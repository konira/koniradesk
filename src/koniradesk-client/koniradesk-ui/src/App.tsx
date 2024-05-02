import { ReactNode } from 'react';
import styled from '@emotion/styled'

const PageContainer = styled.div`  
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: start;
  width: 100vw;
  height: 100vh;
  padding: 10px 10px 10px 10px;  
  background-color: #242424;  
`
const AppContainer = styled.div`
  width: 100vw;
  height: 100vh;
  padding: 0;
  margin: 0;
`

export default function App({ children }: { children: ReactNode }) {
  return (
    <AppContainer>      
      <PageContainer>  
        {children}                  
      </PageContainer>
    </AppContainer>
  );
}
