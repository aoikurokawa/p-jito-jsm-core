import React from "react";
import { Modal } from "antd";

const DisplayModal = ({ isModalVisible, handleOk, handleCancel, title, hash }) => {
  return (
    <Modal
      title={title}
      visible={isModalVisible}
      onOk={handleOk}
      onCancel={handleCancel}
      cancelButtonProps={{ disabled: false }}
    >
      {hash && <p style={{ color: "black" }}>Transaction Hash: {hash}</p>}
    </Modal>
  );
};

export default DisplayModal;
