# vasttrafik-cli

Allows planning trips in the Västtrafik public transport network.

## Usage 
Make sure to surround multi word stations with quotation marks. 

### Planning trips
`cargo run -- plan -f [departure station] -t [arrival station]`
This will print the first trip the travel planner proposes. In the future you will be able to print multiple trips. 

### Searching for stations
`cargo run -- search [station name]`
This will search for stations. Prints all stations and places somewhat matching the search.