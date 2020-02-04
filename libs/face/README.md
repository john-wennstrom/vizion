### Work pipeline
1. Detect presence and location of faces in an image
2. Compute 128-d feature vector (embeddings) that quantify each face in image
3. Train a support vector machine (SVM) on top of embeddings.
4. Recognize faces in images and video streams

For face recognition we need three images, one `anchor`, one `positive` and one `negative`. Anchor and positive is images of same person while negative is another person. We need to tweak the weights of the network via `triplet loss function` such that:
1. The 128-d embeddings of the anchor and positive image lie closer together
2. At the same time, pushing the embeddings of the negative image father away
