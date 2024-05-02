import styled from '@emotion/styled'
import IDLocalPanel from '../components/IDLocalPanel';
import IDConnectPanel from '../components/IDConnectionPanel';


const HomeContainer = styled.div`  
  display: flex;
  flex-direction: row;
  justify-content: start;
  align-items: start;
  text-align: left;
  width: 100%;  
  height: 100%;  
  `
const LeftContainer = styled.div`  
  display: flex;
  gap: 5px;
  flex-direction: column;
  justify-content: start;
  align-items: start;    
  max-width: 200px;
  border-right: 1px solid #696767;
  height: 100%;  
  `
const RightContainer = styled.div`
    display: flex;
    flex-direction: column;
    justify-content: start;
    align-items: start;
    gap: 5px;
    padding: 10px;
    height: 100%;
    width: 100%;    
`
export default function Home() {
    return (
        <HomeContainer>
            <LeftContainer>
               <IDLocalPanel />
            </LeftContainer>
            <RightContainer>
               <IDConnectPanel />
            </RightContainer>
        </HomeContainer>
    );
}