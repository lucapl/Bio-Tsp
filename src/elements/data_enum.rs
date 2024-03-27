
// special thanks to chat gpt for this one!

#[derive(Debug)]
pub enum EdgeWeightType {
    Explicit,
    Euc2D,
    Euc3D,
    Max2D,
    Max3D,
    Man2D,
    Man3D,
    Ceil2D,
    Geo,
    Att,
    XRay1,
    XRay2,
    Special,
    None,
}

#[derive(Debug)]
pub enum EdgeWeightFormat {
    Function,
    FullMatrix,
    UpperRow,
    LowerRow,
    UpperDiagRow,
    LowerDiagRow,
    UpperCol,
    LowerCol,
    UpperDiagCol,
    LowerDiagCol,
    None,
}

#[derive(Debug)]
pub enum EdgeDataFormat {
    EdgeList,
    AdjList,
    None,
}

#[derive(Debug)]
pub enum NodeCoordType {
    TwoDCoords,
    ThreeDCoords,
    NoCoords,
}

#[derive(Debug)]
pub enum DisplayDataType {
    CoordDisplay,
    TwoDDisplay,
    NoDisplay,
}