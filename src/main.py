import os.path
import sys

import wx

from keymap import Keymap
from piano import Piano

class PianoApp(wx.App):
    def __init__(self, *args, **kwargs):
        self.frozen = getattr(sys, 'frozen', False)
        if self.frozen:
            self.app_dir = os.path.dirname(os.path.abspath(__file__))
        else:
            self.app_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.start_note = 48
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
        self.keymap.set_start_note(self.start_note)
        self.keymap.organize_notes()

    def init_piano(self):
        self.piano = Piano()

    def init_ui(self):
        self.mainFrame = wx.Frame(parent = None, id = -1, title = 'Virtual Piano')
        self.Bind(wx.EVT_KEY_DOWN, self.on_key_down)
        self.Bind(wx.EVT_KEY_UP, self.on_key_up)
        self.mainFrame.Show(True)

    def on_key_down(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            return
        if note not in self.notes_on:
            self.notes_on.append(note)
            self.piano.note_on(note)

    def on_key_up(self, evt):
        note = self.get_note_from_key_event(evt)
        if note is None:
            return
        if note in self.notes_on:
            self.notes_on.remove(note)
            self.piano.note_off(note)

    def get_note_from_key_event(self, evt):
        key = evt.GetUnicodeKey()
        if key != wx.WXK_NONE:
            key = chr(key)
            return self.keymap[key] if key in self.keymap else None


if __name__ == '__main__':
    app = PianoApp()
    app.MainLoop()
