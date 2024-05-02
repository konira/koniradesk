import { Button, Card, CardActions, CardContent, Input, Typography } from "@mui/material";

export default function IDConnectPanel() {
    return (
        <Card variant="outlined" >
        <CardContent  >
            <Typography variant="h6" align="left" gutterBottom color={'text.primary'}>
                Controle um Computador Remoto
            </Typography>
            <Input placeholder="ID do Computador"
                color={"secondary"}
                size='medium'                            
                fullWidth      
                autoFocus                                                                                          
                type='text'
            />
        </CardContent>
        <CardActions sx={{ textAlign: 'end', alignItems: 'end', justifyContent: 'end' }}>
            <Button variant='outlined' color='info' size="medium" >
                <Typography fontSize={'12px'} variant="subtitle2"  gutterBottom color="text.primary" >
                    Transferir arquivos
                </Typography>
            </Button>
            <Button variant='contained' color='info' size="medium" >
                <Typography fontSize={'12px'} variant="subtitle2"  gutterBottom color="text.primary" >
                    Conectar
                </Typography>
            </Button>
        </CardActions>
    </Card>
    );
}