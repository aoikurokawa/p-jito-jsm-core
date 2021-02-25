import React from "react";
import { Modal } from "antd";

const DisplayModal = ({isModalVisible, handleOk, handleCancel, title, hash}) => {
  return (
    <Modal
      title={title}
      visible={isModalVisible}
      onOk={handleOk}
      onCancel={handleCancel}
    >
      <p>Transaction Hash: {hash}</p>
      <p></p>
      <p>Some contents...</p>
    </Modal>
  );
};

export default DisplayModal;
