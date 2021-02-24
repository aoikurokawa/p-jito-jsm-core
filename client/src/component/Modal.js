import React from "react";
import { Modal } from "antd";

const DisplayModal = ({isModalVisible, handleOk, handleCancel, title}) => {
  return (
    <Modal
      title={title}
      visible={isModalVisible}
      onOk={handleOk}
      onCancel={handleCancel}
    >
      <p></p>
      <p>Some contents...</p>
      <p>Some contents...</p>
    </Modal>
  );
};

export default DisplayModal;
