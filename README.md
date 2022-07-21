# hothouse
interface and algorithm for building hothouses out of a reused set of windows


**State:** development<br>
**algo:** optimization process<br>
**webserver:** init (actix-web)<br>
**interface:** init<br>

#### Rectangles packing algothm implementation in rust. 
Based on a set of predefined rectangles ( windows ) of dimension H x W with a stock of n that are used for the 4 surfaces ( sides ) of the container.<br>
Top surface, is considered seperatly with a different set of rectangles.<br>

Try to maximise covered surface of all 4 + 1 surfaces with **rectangles packing** and  : 
- testing different order of construction : Permutation(4) ~ might be unecessary 
- shifting rectangles order ~ very time consumming method, used due to low dimensions of the surfaces, could lead to better solution by avoiding largest rectangles ( still to be proved)


#### server in actix-web
