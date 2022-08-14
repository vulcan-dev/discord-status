# discord-status
A simple program that allows you to add buttons to your Discord Status

## How to use
1. Download the latest version of the program from [here](https://github.com/vulcan-dev/discord-status/releases/tag/v0.1) (.exe for Windows)
2. Create an application on the [dashboard](https://discordapp.com/developers/applications)
3. Copy the `Application ID`
4. Run the program to generate `config.json`
5. Enter the `Application ID` you copied in step 2 into the `Application ID` field
6. Setup the buttons
7. Run the program again

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

## Image Example
![](https://imgur.com/1hLoy5F.png)