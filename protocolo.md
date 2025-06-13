### **Protocolo de Eleição Web - Proposta de Arquitetura**

#### **1. Princípios Filosóficos**
* **Segurança Apesar das Pessoas:** O sistema não depende da boa-fé ou da competência dos participantes, mas é forçado a ser seguro por suas regras matemáticas e arquiteturais.
* **Sigilo Absoluto do Voto:** O conteúdo do voto de um indivíduo nunca é revelado a nenhuma parte, em nenhum momento.
* **Integridade e Verificabilidade:** Cada eleitor vota apenas uma vez e a contagem é publicamente auditável por qualquer pessoa no mundo, garantindo a exatidão do resultado.
* **Resiliência Descentralizada:** O sistema não possui ponto único de falha e é resistente a ataques de negação de serviço e censura.

#### **2. Atores do Sistema**
* **Proponente:** Entidade que inicia a proposta de uma eleição.
* **Eleitor:** Cidadão com direito a voto, interagindo através de um software cliente.
* **Nó Verificador:** Entidades independentes (idealmente com interesses conflitantes OU com interesses alinhados a favor do processo) responsáveis por validar a elegibilidade dos votos e participar da apuração.


### **3. Escopo, Objetivos e Não-Objetivos**


#### **3.1. Objetivos Fundamentais**

O principal objetivo do protocolo é **garantir a integridade criptográfica e o sigilo do processo de votação e apuração**. Para isso, ele foi projetado para cumprir estritamente os seguintes requisitos:

1.  **Sigilo do Voto:** Garantir que o conteúdo de um voto individual jamais seja revelado a qualquer parte, incluindo os Nodos Verificadores, em qualquer fase do processo. A apuração ocorre sobre dados criptografados, e apenas o resultado agregado final é decriptado.
2.  **Integridade do Voto:** Assegurar que cada eleitor, devidamente autenticado conforme a lista de elegibilidade, possa ter apenas um voto computado no resultado final. O protocolo deve impedir tanto votos duplicados quanto a alteração de votos após sua publicação na DAG.
3.  **Auditabilidade Universal:** Permitir que qualquer pessoa ou entidade no mundo possa, de forma independente, verificar a exatidão da apuração final a partir dos dados públicos da DAG, sem comprometer o sigilo dos votos individuais.
4.  **Resiliência e Disponibilidade:** Oferecer uma arquitetura descentralizada, sem ponto único de falha, que seja intrinsecamente resistente a ataques de negação de serviço (DDoS) e a tentativas de censura de votos por parte de atores maliciosos.

#### **3.2. Não-Objetivos (Fora do Escopo)**

Um protocolo robusto também se define pelo que ele não se propõe a resolver. Os seguintes aspectos são considerados responsabilidades do ecossistema que implementa o protocolo, e não do protocolo em si:

1.  **Gestão de Identidade e Registro de Eleitores:** O protocolo assume a preexistência de uma "lista de chaves públicas dos eleitores elegíveis". A criação, validação, segurança e o mapeamento entre um cidadão e sua chave pública são responsabilidades políticas e administrativas do **Proponente** da eleição, sendo um processo independente e anterior ao início do protocolo.
2.  **Segurança do Dispositivo Cliente (Client-Side Security):** O protocolo define as regras para a formatação, criptografia e transmissão de dados, mas não o software específico que roda no dispositivo do eleitor. A segurança do ambiente do usuário (contra malware, vírus, etc.) e a qualidade da implementação do software cliente são cruciais para a segurança de ponta a ponta, mas sua garantia está fora do escopo deste documento.
3.  **Coerção e Venda de Votos:** O protocolo protege o sigilo do voto *tecnicamente*, mas não pode resolver cenários de coação que ocorrem fora do sistema (ex: um eleitor forçado a votar na presença de um coator). A mitigação destes problemas de natureza social e de segurança física cabe a mecanismos externos e políticas complementares adotadas pelo **Proponente**.
4.  **Interface de Usuário (UI/UX) e Método de Decisão:** O protocolo é agnóstico em relação à forma como as opções são apresentadas ao eleitor. Ele entrega uma matriz de preferências agregada ($M_{final}$) como resultado final e auditável. A escolha de usar o método Schulze, Pluralidade, ou qualquer outro algoritmo de decisão sobre essa matriz é uma decisão de governança a ser definida pelo **Proponente** na Fase 1 da eleição.


#### **3.3. O Papel do Protocolo**

Em suma, este protocolo deve ser entendido como um **motor de apuração seguro e transparente**. Ele não é a eleição inteira, mas seu núcleo matemático. Sua função é receber votos de fontes já consideradas autênticas e entregar um resultado agregado que é matematicamente verificável por todos.

---

### **3. Fases do Protocolo**

#### **Fase 1: Setup e Governança**
1.  **Proposta:** O Proponente publica na DAG (Directed Acyclic Graph) a definição da eleição: candidatos, método de decisão ranqueado (ex: Schulze), período de votação, lista de chaves públicas dos eleitores elegíveis, e a lista dos Nodos Verificadores propostos.
2.  **Escrutínio Público:** A proposta entra em um período de "escrutínio", durante o qual a comunidade e outras entidades podem auditar as regras. A eleição só se torna ativa após ser ratificada (assinada) pela maioria dos Nodos Verificadores citados na proposta. Os nodos que não ratificarem não podem participar do processo nas fases seguintes.
3.  **Geração da Chave da Eleição:** Os Nodos Verificadores realizam um protocolo de Geração de Chave Distribuída (*Distributed Key Generation*) para criar coletivamente uma única **chave pública da eleição ($PK_{eleição}$)**. A chave privada correspondente já nasce fragmentada, com cada verificador detendo apenas uma parte, sendo impossível reconstruí-la em um único local.

#### **Fase 2: Votação (Ação do Eleitor)**
1.  **Criação da Cédula:** O cliente do eleitor gera uma **matriz de preferência pareada ($m_i$)** com base no ranking de candidatos escolhido pelo usuário.
2.  **Criptografia:** O cliente criptografa a matriz $m_i$ usando a chave pública da eleição ($PK_{eleição}$), resultando no pacote de voto criptografado, $E(m_i)$.
3.  **Assinatura:** O eleitor assina o pacote $E(m_i)$ com sua chave privada pessoal, provando sua identidade e autenticidade.
4.  **Submissão Múltipla e Redundante:** Para garantir velocidade e resistência à censura, o cliente envia o pacote de voto final (assinado e criptografado) para **múltiplos Nodos Verificadores** simultaneamente.

#### **Fase 3: Verificação e Consenso na DAG**
1.  **Verificação de Elegibilidade:** Um Nó Verificador recebe o pacote. Ele verifica a assinatura do eleitor contra a lista pública de eleitores válidos.
2.  **Publicação:** Se a assinatura for válida, o Verificador assina o pacote (atestando a elegibilidade) e o publica na DAG.
3.  **Redundância e Resolução de Conflitos:** 
    O protocolo utiliza o mecanismo de consenso da DAG para impor a regra de "um voto por eleitor" no nível da rede. Qualquer tentativa de publicar múltiplos pacotes de voto assinados pelo mesmo eleitor é tratada como um conflito de "gasto duplo".

Seja a tentativa maliciosa (um eleitor enviando votos com conteúdo diferente) ou por design para resiliência (múltiplos verificadores publicando o mesmo pacote de voto E(mi​)), o resultado no consenso é o mesmo: a rede, através de seu peso cumulativo, eventualmente confirmará apenas uma dessas transações. Todas as outras publicações concorrentes se tornarão ramos órfãos e serão permanentemente ignoradas pela rede, garantindo que apenas um voto por eleitor possa ser incluído na apuração. Em casos maliciosos, votos diferentes de um mesmo eleitor podem ser desconsiderados ou escolhidos ao acaso, de acordo com o especificado na proposta definida na **Fase 1**.

#### **Fase 4: Apuração e Auditoria Universal**
1.  **Apuração Homomórfica:** Após o fim do período de votação, **qualquer pessoa** pode baixar os dados da DAG. O software de apuração identifica todos os pacotes de voto únicos (descartando duplicatas exatas publicadas por diferentes verificadores) e os **soma homomorficamente enquanto ainda estão criptografados**. Isso resulta em uma única matriz agregada criptografada: $E(M_{final}) = \sum E(m_i)$.
2.  **Decriptação Única:** Os Nodos Verificadores colaboram para realizar **uma única decriptação em limiar** sobre $E(M_{final})$, revelando a matriz de preferência pareada final e agregada, $M_{final}$.
3.  **Resultado e Auditoria:**
    * A matriz $M_{final}$ é pública. Os votos individuais **nunca foram decriptados**.
    * Qualquer cidadão, partido ou observador pode executar o algoritmo de decisão (ex. Schulze, Tideman) sobre a matriz pública $M_{final}$ para calcular e verificar independentemente o resultado da eleição.

---
