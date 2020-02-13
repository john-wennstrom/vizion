## Text recognition


### Creating Dataset
Get open source dataset for segmentation part:
* Total-Text-Dataset
* COCO-Text
* The Street View Text Dataset
* Table Ground Truth
Open source dataset for text recognition:
* Google FSNS
* Synthetic Word
* MNIST Handwritten
* Born-Digital Images

### Input Image
### Image preprocessing
Unskew, remove noise, occlusion and more.
### Segmentation
Using algorithms as EAST, CTPN, Deep direct, SSTD or faster R-CNN.

#### CNN (Convolutional Neural Network)
Apply CNN layers to input image for extracting features (Concolutional feature extraction).
### Restructuring
#### RNN (Recurrent Neural Network)
Produce encoded features of the image. Calculate probability values for each input feature. Use NLP (Natural Language processing) to train the RNN models. Predict sequental ouput.
#### CTC (Connectionist Temporal Classification)
Loss function.
#### Ouput text
