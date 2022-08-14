# discord-status
A simple program that allows you to add buttons to your Discord Status

## How to use
1. Create an application on the [dashboard](https://discordapp.com/developers/applications)
2. Copy the `Application ID`
3. Run the program, a file called `config.json` will be created in the same directory as the program
4. Enter the `Application ID` you copied in step 2 into the `Application ID` field
5. Setup the buttons

## JSON Example
```json
{
  "application_id": "YOUR_APPLICATION_ID",
  "details": "Anything you want, you can even delete this field",
  "buttons": [
    {
      "name": "Github",
      "url": "https://github.com/vulcan-dev"
    },
    {
      "name": "SurrounDead Game",
      "url": "https://discord.com/invite/surroundead"
    }
  ]
}
```