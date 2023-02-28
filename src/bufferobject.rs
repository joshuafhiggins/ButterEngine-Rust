use crate::vertex::Vertex;

pub trait BufferObject {
    fn cleanup(&mut self);
    fn bind(&mut self);
    fn unbind(&mut self);
}

pub struct VAO {

}

pub struct IBO {

}

pub struct VBO {

}

pub struct TBO {

}

// public int storeData(FloatBuffer buffer, int index, int size) {
//     int bufferID = glGenBuffers();
//     glBindBuffer(GL_ARRAY_BUFFER, bufferID);
//     glBufferData(GL_ARRAY_BUFFER, buffer, GL_STATIC_DRAW);
//     glVertexAttribPointer(index, size, GL_FLOAT, false, 0, 0);
//     glBindBuffer(GL_ARRAY_BUFFER, 0);
//     return bufferID;
// }

// //Use for IBOs
// public int storeData(IntBuffer indicesBuffer) {
//     int bufferID = glGenBuffers();
//     glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, bufferID);
//     glBufferData(GL_ELEMENT_ARRAY_BUFFER, indicesBuffer, GL_STATIC_DRAW);
//     glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
//     return bufferID;
// }

// * 3
fn getPositionData(vertices: Vec<Vertex>) -> Vec<f32> {
    let positionData: Vec<f32> = Vec::with_capacity(vertices.len() * 3);
    for i in 0..vertices.len() {
        positionData.insert(i * 3, vertices.get(i).unwrap().position.x);
        positionData.insert(i * 3 + 1, vertices.get(i).unwrap().position.y);
        positionData.insert(i * 3 + 2, vertices.get(i).unwrap().position.z);
    }
    return positionData;
}

// * 2
fn getTextureCoordData(vertices: Vec<Vertex>) -> Vec<f32> {
    let textureData: Vec<f32> = Vec::new();
    for i in 0..vertices.length {
        textureData[i * 2] = vertices[i].texCoord.x;
        textureData[i * 2 + 1] = vertices[i].texCoord.y;
    }
    return textureData;
}