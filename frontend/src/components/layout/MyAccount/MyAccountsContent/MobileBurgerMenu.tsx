import React from "react";
import styled from 'styled-components';
import {Col, Container, Modal, Row,} from "react-bootstrap";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import {faChevronRight} from "@fortawesome/free-solid-svg-icons";
import {GridItem} from "./GridItem";

interface MobileBurgerMenuProps {
  className?: string;
  nftInfo?: any;
}

export const MobileBurgerMenuBase = (props: MobileBurgerMenuProps) => {
  const {
    className,
    nftInfo
  } = props;


  return (
    <div className={className}>
      <Row className="h-auto grid-row">
        {nftInfo.map((value: any, index: any) => {
          return <Col key={index}
                      xl={{span:4}}
                      lg={{span: 6}}
                      md={{span: 6}}
                      xs={{span: 12}}
                      className="d-flex flex-column justify-content-start align-self-start align-content-center align-items-center col mb-4">
            <GridItem nftValue={value}/>
          </Col>
        })}
      </Row>
    </div>
  )
}


export const MobileBurgerMenu = styled(MobileBurgerMenuBase)`
  
  
`;