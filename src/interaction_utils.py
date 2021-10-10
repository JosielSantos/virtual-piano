import wx

def edit_dialog(parent, title, description, default_value = "", style=wx.OK | wx.CANCEL):
    """Displays a text dialog and waits the user to input a value.

    Parameters:
        parent: the window the user came from.
        Title: the title of the dialog.
        Description: The text that will be displayed on the edit field.
        Default_value: a value that will be set on the edit field when the user enters the dialog.
        Style: the style of the dialog.

        Returns two values:
            The first value is a boolean that specifies if the user pressed the OK button.
            The second value will depend if the OK button was pressed. If the OK button was pressed, it returns the entered text. Else it returns an empty string."""
    with wx.TextEntryDialog(parent, description, title, style = style) as dialog:
        if default_value:
            dialog.SetValue(default_value)

        if dialog.ShowModal() == wx.ID_OK:
            return (True, dialog.GetValue())
        else:
            return (False, "")

def message_dialog(parent, title, description, style = wx.OK):
    """Displays a simple message to the user.

    Parameters:
        parent: the window the user came from.
        Title: the title of the dialog.
        Description: The text that will be displayed on the dialog.
        Style: the style of the dialog.

        Returns the button the user pressed."""
    with wx.MessageDialog(parent, description, title, style = style) as dialog:
        return dialog.ShowModal()
