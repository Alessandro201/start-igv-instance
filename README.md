# Start an IGV instance

The program will create a local webserver with IGV at `http://localhost:8000`, and will make the files in the folder you select available at `http://localhost:8001/`.
It will also automatically open an IGV instance in your preferred browser for convenience.

Once you are done, to terminate the webservers you can just close the terminal.

In `resources` folder you can find the IGV application, taken directly from the [GitHub](https://github.com/igvteam/igv-webapp) page. The only thing I added is the (GENCODE)[https://www.gencodegenes.org/human/] annotation v46 for the human genome in the `resources/igv-webapp/resources/tracks/gencode_annotations` folder. 
However, if you prefer to use your own version of IGV you can follow the instructions on their github page on how to download and compile it, and then you just need to place it inside the `resources` folder. If you would like to add your own tracks you can use the commands [here](#resources/igv-webapp/resources/tracks/gencode_annotations/readme) to create an index for you own files. 

On the [IGV documentation](https://igv.org/doc/webapp/#) you can find useful info on how to better use IGV, and how to start your own instance.
