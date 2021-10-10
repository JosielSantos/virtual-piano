import sys

import wx

from config import Config
import constants
from interaction_utils import edit_dialog, message_dialog
from util import app, char
import midi
from piano import Piano

class PianoApp(wx.App):
    config = None
    functions_keymap = {}
    multi_voice = False
    active_channels = []
    current_channel = 0

    def OnInit(self):
        try:
            self.load_config()
            self.init_piano()
            self.init_ui()
            return True
        except Exception as exc :
            wx.MessageBox(str(exc), 'Erro', wx.OK | wx.ICON_ERROR)
            return False

    def load_config(self):
        self.config = Config(app.file_path('config.ini'))

    def init_piano(self):
        self.midi = midi.Midi()
        midi_output_driver = self.config.get_midi_output_driver(constants.MIDI_OUTPUT_DEFAULT_DRIVER)
        if midi_output_driver == constants.MIDI_OUTPUT_DEFAULT_DRIVER:
            self.midi_output = midi.Output(self.midi.get_default_output_id(), 0)
        else:
            raise ValueError('MIDI driver inexistente')
        self.piano = Piano(self.midi_output)
        self.active_channels.append(0)
        self.piano.set_instrument(0, 0)

    def init_ui(self):
        self.mainFrame = wx.Frame(parent = None, id = -1, title = 'Virtual Piano')
        self.mainFrame.Bind(wx.EVT_KEY_DOWN, self.on_key_down)
        self.mainFrame.Bind(wx.EVT_KEY_UP, self.on_key_up)
        self.functions_keymap = {
            wx.WXK_RIGHT: lambda evt: self.piano.next_instrument(self.current_channel),
            wx.WXK_LEFT: lambda evt: self.piano.previous_instrument(self.current_channel),
            wx.WXK_UP: lambda evt: self.tone_change_up(evt),
            wx.WXK_DOWN: lambda evt: self.tone_change_down(evt),
            wx.WXK_PAGEUP: lambda evt: self.next_channel(),
            wx.WXK_PAGEDOWN: lambda evt: self.previous_channel(),
            wx.WXK_DELETE: lambda evt: self.delete_current_channel(),
            wx.WXK_F2: lambda evt: self.select_instrument_by_number(self.current_channel),
            wx.WXK_F8: lambda evt: self.piano.volume_down(self.current_channel),
            wx.WXK_F9: lambda evt: self.piano.volume_up(self.current_channel),
            wx.WXK_BACK: self.toggle_multi_voice,
            wx.WXK_TAB: self.pan,
        }
        self.mainFrame.Show(True)

    def on_key_down(self, evt):
        key = evt.GetKeyCode()
        if key in self.functions_keymap:
            self.functions_keymap[key](evt)
        if self.multi_voice:
            for channel_number in self.active_channels:
                note = self.get_note_from_key_event(evt, channel_number)
                if note is not None:
                    self.piano.note_on(note, channel_number)
        else:
            note = self.get_note_from_key_event(evt, self.current_channel)
            if note is not None:
                self.piano.note_on(note, self.current_channel)

    def on_key_up(self, evt):
        if self.multi_voice:
            for channel_number in self.active_channels:
                note = self.get_note_from_key_event(evt, channel_number)
                if note is not None:
                    self.piano.note_off(note, channel_number)
        else:
            note = self.get_note_from_key_event(evt, self.current_channel)
            if note is not None:
                self.piano.note_off(note, self.current_channel)

    def get_note_from_key_event(self, evt, channel_number):
        key = evt.GetUnicodeKey()
        if key != wx.WXK_NONE:
            if key > 127:
                key = char.unicode_workaround(chr(key).encode('utf-8'))
            return self.piano.get_note(key, channel_number)

    def toggle_multi_voice(self, evt):
        self.piano.all_notes_off()
        self.multi_voice = not self.multi_voice

    def pan(self, evt):
        back = True if evt.GetModifiers() == wx.MOD_SHIFT else False
        self.piano.pan(back, self.current_channel)

    def next_channel(self):
        if self.current_channel == 15:
            return
        self.piano.all_notes_off()
        self.current_channel += 1
        if not self.current_channel in self.active_channels:
            self.active_channels.append(self.current_channel)
            self.piano.get_channel(self.current_channel)

    def previous_channel(self):
        if self.current_channel == 0:
            return
        self.piano.all_notes_off()
        self.current_channel -= 1

    def delete_current_channel(self):
        self.piano.delete_channel(self.current_channel)
        self.active_channels.remove(self.current_channel)
        self.current_channel -= 1

    def tone_change_up(self, evt):
        key_modifier = evt.GetModifiers()
        if key_modifier ==  wx.MOD_SHIFT:
            self.piano.semitone_up(1, self.current_channel)
        else:
            self.piano.octave_up(self.current_channel)

    def tone_change_down(self, evt):
        key_modifier = evt.GetModifiers()
        if key_modifier ==  wx.MOD_SHIFT:
            self.piano.semitone_down(1, self.current_channel)
        else:
            self.piano.octave_down(self.current_channel)

    def select_instrument_by_number(self, target_channel):
        current_instrument = str(self.piano.get_instrument_for_channel(target_channel))
        pressed_ok, selected_instrument = edit_dialog(self.mainFrame, "Instrument", "Enter instrument number for channel %d (from 0 to 127):" % target_channel, current_instrument)

        if pressed_ok:
            try:
                instrument_number = int(selected_instrument)
            except ValueError:
                message_dialog(self.mainFrame, "Error", "Instrument not a number")
                return

            if instrument_number < 0 or instrument_number > 127:
                message_dialog(self.mainFrame, "Error", "Instrument number not in range from 0 to 127")
                return

            self.piano.set_instrument(instrument_number, target_channel)

if __name__ == '__main__':
    app = PianoApp()
    app.MainLoop()
