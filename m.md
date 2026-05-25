## Day-2
Update the UI and make it more user-friendly


### update
- added ai can use tools like web search, file read, current time, etc.
- enhnaced the UI more user friendly
- remove terminal based signup system and add web based signup system
- support user name give, user can give pet name and pet type, hack club api use, etc.
- enhanced the model response


## Tools explanation
- web search: using duckduckgo search api, search the web for information, limits: some times wrong result answer
- file read: using rust standard library to read a file, give readed code into the model as context, limits: only read text file
- current time: using chrono library to get the current time.user prompt will be like "what is the current time?"  that time contain auto show time
- etc.



### todo
- support more tools like weather, calculator, etc.
- improve the UI
- add more features
- add more pets




thanks for reading this file