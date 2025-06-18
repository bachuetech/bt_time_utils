# Project Title
BT TIME UTILS

## Description
Simple time formatting utility. Returns current date and time using the requested format. 

Uses chrono formatting: 
https://docs.rs/chrono/latest/chrono/format/strftime/index.html

## Usage
```
let (c_time, c_date) = get_current_time_and_date();
let (c_time, c_date) = get_formatted_time_and_date("%H:%M","%B %d, %Y");

```

## Version History
* 0.1.0
    * Initial Release
* 0.1.1
    * Added get formated date    

## License
GPL-3.0-only