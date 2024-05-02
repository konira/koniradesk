import styled from '@emotion/styled'
import { Typography } from '@mui/material';

const ID = styled.div`        
  border-left: 4px solid #3f51b5;
  font-weight: bold;
  border-left: 4px solid ter;
  padding-left: 10px;
  &:hover {
    color: secundary;
  }
`
export default function IDLocalPanel() {

    return (
        <div>
            <Typography variant="h6" align="left" gutterBottom color={'text.primary'}>
                Seu Computador
            </Typography>
            <Typography fontSize={'12px'} variant="subtitle2" align="left" gutterBottom color="text.secondary" >
                Seu computador pode ser acessado com este ID e senha.
            </Typography>
            <ID>
                <Typography fontSize={'12px'} variant="subtitle2" align="left" gutterBottom color="text.secondary" >
                    Seu ID:
                </Typography>
                <Typography variant="h6" align="left" gutterBottom color={'text.primary'}>
                    123 456 789
                </Typography>
            </ID>
            <ID>
                <Typography fontSize={'12px'} variant="subtitle2" align="left" gutterBottom color="text.secondary" >
                    Senha de uso único:
                </Typography>
                <Typography variant="h6" align="left" gutterBottom color={'text.primary'}>
                    ********
                </Typography>
            </ID>
        </div>
    );
}