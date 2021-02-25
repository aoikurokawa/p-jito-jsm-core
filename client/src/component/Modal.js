import React from "react";
import { Modal } from "antd";

const DisplayModal = ({isModalVisible, handleOk, title, hash}) => {
  return (
    <Modal
      title={title}
      visible={isModalVisible}
      onOk={handleOk}
    >
      <p style={{color: "black"}}>Transaction Hash: {hash}</p>
    </Modal>
  );
};

export default DisplayModal;
