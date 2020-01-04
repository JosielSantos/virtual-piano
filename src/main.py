import os.path
import sys

import wx

from char_utils import unicode_workaround
import midi
from note import NotesManager
from piano import Piano

class PianoApp(wx.App):
    functions_keymap = {}
    multi_voice = False

    def __init__(self, *args, **kwargs):
        self.frozen = getattr(sys, 'frozen', False)
        if self.frozen:
            self.app_dir = os.path.dirname(os.path.abspath(__file__))
        else:
            self.app_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        wx.App.__init__(self, *args, **kwargs)

    def OnInit(self):
        self.init_piano()
        self.init_ui()
        return True

    def init_piano(self):
        keymap_filename = 'pianoeletronico.kmp'
        notes_manager = NotesManager()
        notes_manager.load_file(os.path.join(self.app_dir, 'keymaps', keymap_filename))
        self.midi = midi.Midi()
        self.midi_output = midi.Output(self.midi.get_default_output_id(), 0)
        self.piano = Piano(notes_manager, self.midi_output)
        self.piano.set_instrument(0, 0)

    def init_ui(self):
        self.mainFrame = wx.Frame(parent = None, id = -1, title = 'Virtual Piano')
        self.Bind(wx.EVT_KEY_DOWN, self.on_key_down)
        self.Bind(wx.EVT_KEY_UP, self.on_key_up)
        self.functions_keymap = {
            wx.WXK_RIGHT: lambda evt: self.piano.next_instrument(),
            wx.WXK_LEFT: lambda evt: self.piano.previous_instrument(),
            wx.WXK_UP: lambda evt: self.piano.octave_up(),
            wx.WXK_DOWN: lambda evt: self.piano.octave_down(),
            wx.WXK_PAGEUP: lambda evt: self.piano.next_channel(),
            wx.WXK_PAGEDOWN: lambda evt: self.piano.previous_channel(),
            wx.WXK_DELETE: lambda evt: self.piano.delete_current_channel(),
            wx.WXK_F8: lambda evt: self.piano.volume_down(),
            wx.WXK_F9: lambda evt: self.piano.volume_up(),
            wx.WXK_BACK: self.toggle_multi_voice,
            wx.WXK_TAB: self.pan,
        }
        self.mainFrame.Show(True)

    def on_key_down(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            key = evt.GetKeyCode()
            if key in self.functions_keymap:
                self.functions_keymap[key](evt)
        else:
            if self.multi_voice:
                self.piano.note_on_multi(note)
            else:
                self.piano.note_on(note)

    def on_key_up(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            return
        if self.multi_voice:
            self.piano.note_off_multi(note)
        else:
            self.piano.note_off(note)

    def get_note_from_key_event(self, evt):
        key = evt.GetUnicodeKey()
        if key != wx.WXK_NONE:
            if key > 127:
                key = unicode_workaround(chr(key).encode('utf-8'))
            return self.piano.notes_manager[key] if key in self.piano.notes_manager else None

    def toggle_multi_voice(self, evt):
        self.piano.all_notes_off()
        self.multi_voice = not self.multi_voice

    def pan(self, evt):
        back = True if evt.GetModifiers() == wx.MOD_SHIFT else False
        self.piano.pan(back)


if __name__ == '__main__':
    app = PianoApp()
    app.MainLoop()
