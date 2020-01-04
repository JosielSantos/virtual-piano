import os.path
import sys

import wx

from keymap import Keymap
from piano import Piano

class PianoApp(wx.App):
    functions_keymap = {}
    multi_voice = False
    current_channel = 0
    channels = {}

    def __init__(self, *args, **kwargs):
        self.frozen = getattr(sys, 'frozen', False)
        if self.frozen:
            self.app_dir = os.path.dirname(os.path.abspath(__file__))
        else:
            self.app_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.start_note = 48
        self.channels[0] = self.make_channel(0, 127)
        wx.App.__init__(self, *args, **kwargs)

    def OnInit(self):
        self.notes_on = []
        self.load_keymap()
        self.init_piano()
        self.init_ui()
        return True

    def load_keymap(self):
        keymap_filename = 'pianoeletronico.kmp'
        self.keymap = Keymap()
        self.keymap.load_file(os.path.join(self.app_dir, 'resources', 'keymaps', keymap_filename))
        self.organize_notes()

    def organize_notes(self):
        self.keymap.set_start_note(self.start_note)
        self.keymap.organize_notes()

    def init_piano(self):
        self.piano = Piano()
        self.set_instrument(self.channels[0]['instrument'], self.current_channel)

    def set_instrument(self, instrument_id, channel):
        self.channels[channel]['instrument'] = instrument_id
        self.piano.set_instrument(instrument_id, channel)

    def init_ui(self):
        self.mainFrame = wx.Frame(parent = None, id = -1, title = 'Virtual Piano')
        self.Bind(wx.EVT_KEY_DOWN, self.on_key_down)
        self.Bind(wx.EVT_KEY_UP, self.on_key_up)
        self.functions_keymap = {
            wx.WXK_RIGHT: self.next_instrument,
            wx.WXK_LEFT: self.previous_instrument,
            wx.WXK_UP: self.octave_up,
            wx.WXK_DOWN: self.octave_down,
            wx.WXK_PAGEUP: self.next_channel,
            wx.WXK_PAGEDOWN: self.previous_channel,
            wx.WXK_BACK: self.toggle_multi_voice,
            wx.WXK_DELETE: self.delete_current_channel,
            wx.WXK_F8: self.volume_down,
            wx.WXK_F9: self.volume_up,
        }
        self.mainFrame.Show(True)

    def on_key_down(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            key = evt.GetKeyCode()
            if key in self.functions_keymap:
                self.functions_keymap[key]()
        else:
            if note not in self.notes_on:
                self.notes_on.append(note)
                if self.multi_voice:
                    for chan in range(len(self.channels)):
                        self.piano.note_on(note, self.channels[chan]['volume'], chan)
                else:
                    self.piano.note_on(note, self.channels[self.current_channel]['volume'], self.current_channel)

    def on_key_up(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            return
        if note in self.notes_on:
            self.notes_on.remove(note)
            if self.multi_voice:
                for chan in range(len(self.channels)):
                    self.piano.note_off(note, chan)
            else:
                self.piano.note_off(note, self.current_channel)

    def get_note_from_key_event(self, evt):
        key = evt.GetUnicodeKey()
        if key != wx.WXK_NONE:
            key = chr(key)
            return self.keymap[key] if key in self.keymap else None

    def next_instrument(self):
        if self.channels[self.current_channel]['instrument'] == 127:
            return
            self.all_notes_off()
        self.channels[self.current_channel]['instrument'] += 1
        self.set_instrument(self.channels[self.current_channel]['instrument'], self.current_channel)

    def previous_instrument(self):
        if self.channels[self.current_channel]['instrument'] == 0:
            return
            self.all_notes_off()
        self.channels[self.current_channel]['instrument'] -= 1
        self.set_instrument(self.channels[self.current_channel]['instrument'], self.current_channel)

    def octave_down(self):
        if self.start_note == 0:
            return
        self.all_notes_off()
        self.start_note -= 12
        self.organize_notes()

    def octave_up(self):
        if self.start_note == 120:
            return
        self.all_notes_off()
        self.start_note += 12
        self.organize_notes()

    def next_channel(self):
        if self.current_channel == 15:
            return
        self.all_notes_off()
        self.current_channel += 1
        if self.current_channel not in self.channels:
            self.channels[self.current_channel] = self.make_channel(0, 127)
            self.set_instrument(0, self.current_channel)

    def previous_channel(self):
        if self.current_channel == 0:
            return
        self.all_notes_off()
        self.current_channel -= 1

    def toggle_multi_voice(self):
        self.all_notes_off()
        self.multi_voice = not self.multi_voice

    def delete_current_channel(self):
        self.all_notes_off()
        if self.current_channel > 0:
            self.channels.pop(self.current_channel)
            self.previous_channel()

    def volume_down(self):
        if self.channels[self.current_channel]['volume'] == 0:
            return
        if self.channels[self.current_channel]['volume'] < 10:
            self.channels[self.current_channel]['volume'] = 0
        else:
            self.channels[self.current_channel]['volume'] -= 10

    def volume_up(self):
        if self.channels[self.current_channel]['volume'] == 127:
            return
        if self.channels[self.current_channel]['volume'] >= 118:
            self.channels[self.current_channel]['volume'] = 127
        else:
            self.channels[self.current_channel]['volume'] += 10

    def all_notes_off(self):
        for note in self.notes_on:
            self.notes_on.remove(note)
            for chan in range(self.len(self.channels)):
                self.piano.note_off(note, chan)

    def make_channel(self, instrument_id, volume = 127):
        return {
            'instrument': instrument_id,
            'volume': volume,
        }


if __name__ == '__main__':
    app = PianoApp()
    app.MainLoop()
