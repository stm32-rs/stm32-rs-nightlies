#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `EN` reader - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRINIT` reader - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
pub type CRINIT_R = crate::BitReader;
#[doc = "Field `CRINIT` writer - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
pub type CRINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOARBH` reader - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
pub type NOARBH_R = crate::BitReader;
#[doc = "Field `NOARBH` writer - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
pub type NOARBH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTPTRN` reader - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
pub type RSTPTRN_R = crate::BitReader;
#[doc = "Field `RSTPTRN` writer - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
pub type RSTPTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXITPTRN` reader - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
pub type EXITPTRN_R = crate::BitReader;
#[doc = "Field `EXITPTRN` writer - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
pub type EXITPTRN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HKSDAEN` reader - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
pub type HKSDAEN_R = crate::BitReader;
#[doc = "Field `HKSDAEN` writer - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
pub type HKSDAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HJACK` reader - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
pub type HJACK_R = crate::BitReader;
#[doc = "Field `HJACK` writer - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
pub type HJACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLUSH` writer - RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
pub type RXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRES` reader - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
pub type RXTHRES_R = crate::BitReader;
#[doc = "Field `RXTHRES` writer - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
pub type RXTHRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFLUSH` writer - TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\]
&lt; I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
pub type TXFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTHRES` reader - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
pub type TXTHRES_R = crate::BitReader;
#[doc = "Field `TXTHRES` writer - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
pub type TXTHRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMAEN` reader - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type SDMAEN_R = crate::BitReader;
#[doc = "Field `SDMAEN` writer - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type SDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFLUSH` writer - S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
pub type SFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMODE` reader - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
pub type RMODE_R = crate::BitReader;
#[doc = "Field `RMODE` writer - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
pub type RMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMODE` reader - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
pub type TMODE_R = crate::BitReader;
#[doc = "Field `TMODE` writer - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
pub type TMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMAEN` reader - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type CDMAEN_R = crate::BitReader;
#[doc = "Field `CDMAEN` writer - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
pub type CDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFLUSH` writer - C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
pub type CFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSFSET` writer - frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
pub type TSFSET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
    #[inline(always)]
    pub fn crinit(&self) -> CRINIT_R {
        CRINIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
    #[inline(always)]
    pub fn noarbh(&self) -> NOARBH_R {
        NOARBH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
    #[inline(always)]
    pub fn rstptrn(&self) -> RSTPTRN_R {
        RSTPTRN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
    #[inline(always)]
    pub fn exitptrn(&self) -> EXITPTRN_R {
        EXITPTRN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
    #[inline(always)]
    pub fn hksdaen(&self) -> HKSDAEN_R {
        HKSDAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
    #[inline(always)]
    pub fn hjack(&self) -> HJACK_R {
        HJACK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
    #[inline(always)]
    pub fn rxthres(&self) -> RXTHRES_R {
        RXTHRES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
    #[inline(always)]
    pub fn txthres(&self) -> TXTHRES_R {
        TXTHRES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn sdmaen(&self) -> SDMAEN_R {
        SDMAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    pub fn cdmaen(&self) -> CDMAEN_R {
        CDMAEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I3C enable (whatever I3C is acting as controller/target) - Except registers, the peripheral is under reset (a.k.a. partial reset). - Before clearing EN, when I3C is acting as a controller, all the possible target requests must be disabled using DISEC CCC. - When I3C is acting as a target, software should not disable the I3C, unless a partial reset is needed. In this state, some register fields can not be modified (like CRINIT, HKSDAEN for the I3C_CFGR)"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CFGRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - initial controller/target role This bit can be modified only when I3C_CFGR.EN = 0. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as an I3C target. I3C does not drive SCL line and does not enable SDA pull-up, until it eventually acquires the controller role. Once enabled by setting I3C_CFGR.EN = 1, I3C peripheral initially acts as a controller. It has the I3C controller role, so drives SCL line and enables SDA pull-up, until it eventually offers the controller role to an I3C secondary controller."]
    #[inline(always)]
    #[must_use]
    pub fn crinit(&mut self) -> CRINIT_W<CFGRrs> {
        CRINIT_W::new(self, 1)
    }
    #[doc = "Bit 2 - no arbitrable header after a START (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. - The target address is emitted directly after a START in case of a legacy I2C message or an I3C SDR private read/write message. - This is a more performing option (when is useless the emission of the 0x7E arbitrable header), but this is to be used only when the controller is sure that the addressed target device can not emit concurrently an IBI or a controller-role request (to insure no misinterpretation and no potential conflict between the address emitted by the controller in open-drain mode and the same address a target device can emit after a START, for IBI or MR)."]
    #[inline(always)]
    #[must_use]
    pub fn noarbh(&mut self) -> NOARBH_W<CFGRrs> {
        NOARBH_W::new(self, 2)
    }
    #[doc = "Bit 3 - HDR reset pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame."]
    #[inline(always)]
    #[must_use]
    pub fn rstptrn(&mut self) -> RSTPTRN_W<CFGRrs> {
        RSTPTRN_W::new(self, 3)
    }
    #[doc = "Bit 4 - HDR Exit Pattern enable (when I3C is acting as a controller) This bit can be modified only when there is no on-going frame. This is used to send only the header to test ownership of the bus when there is a suspicion of problem after controller-role hand-off (new controller didn’t assert its controller-role by accessing the previous one in less than Activity State time). The HDR Exit Pattern is sent even if the message header {S/Sr + 0x7E addr + W } is ACKed."]
    #[inline(always)]
    #[must_use]
    pub fn exitptrn(&mut self) -> EXITPTRN_W<CFGRrs> {
        EXITPTRN_W::new(self, 4)
    }
    #[doc = "Bit 5 - High-keeper enable on SDA line (when I3C is acting as a controller) This bit can be modified only when I3C_CFGR.EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn hksdaen(&mut self) -> HKSDAEN_W<CFGRrs> {
        HKSDAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Hot Join request acknowledge (when I3C is acting as a controller) After the NACK, the message continues as initially programmed (the hot-joining target is aware of the NACK and surely emits another hot-join request later on). After the ACK, the message continues as initially programmed. The software is aware by the HJ interrupt (flag I3C_EVR.HJF is set) and initiates the ENTDAA sequence later on, potentially preventing others Hot Join requests with a Disable target events command (DISEC, with DISHJ=1). Independently of the HJACK configuration, further Hot Join request(s) are NACKed until the Hot Join flag, HJF, is cleared. However, a NACKed target can be assigned a dynamic address by the ENTDAA sequence initiated later on by the first HJ request, preventing this target to emit an HJ request again."]
    #[inline(always)]
    #[must_use]
    pub fn hjack(&mut self) -> HJACK_W<CFGRrs> {
        HJACK_W::new(self, 7)
    }
    #[doc = "Bit 8 - RX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software reads and pops a data byte/word from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is to be read by the software either via polling on the flag I3C_EVR.RXFNEF=1 or via interrupt notification (enabled by I3C_IER.RXFNEIE=1). - DMA reads and pops data byte(s)/word(s) from RX-FIFO i.e. reads I3C_RDR or I3C_RDWR register. - A next data byte/word is automatically read by the programmed hardware (i.e. via the asserted RX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CFGRrs> {
        RXDMAEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - RX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written."]
    #[inline(always)]
    #[must_use]
    pub fn rxflush(&mut self) -> RXFLUSH_W<CFGRrs> {
        RXFLUSH_W::new(self, 9)
    }
    #[doc = "Bit 10 - RX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the RX-FIFO level, when the I3C_EVR.RXFNEF flag is set (and consequently if RXDMAEN=1 when is asserted a DMA RX request). RXFNEF is set when 1 byte is to be read in RX-FIFO (i.e. in I3C_RDR). RXFNEF is set when 4 bytes are to be read in RX-FIFO (i.e. in I3C_RDWR)."]
    #[inline(always)]
    #[must_use]
    pub fn rxthres(&mut self) -> RXTHRES_W<CFGRrs> {
        RXTHRES_W::new(self, 10)
    }
    #[doc = "Bit 12 - TX-FIFO DMA request enable (whatever I3C is acting as controller/target) - Software writes and pushes a data byte/word into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register, to be transmitted over the I3C bus. - A next data byte/word is to be written by the software either via polling on the flag I3C_EVR.TXFNFF=1 or via interrupt notification (enabled by I3C_IER.TXFNFIE=1). - DMA writes and pushes data byte(s)/word(s) into TX-FIFO i.e. writes I3C_TDR or I3C_TDWR register. - A next data byte/word transfer is automatically pushed by the programmed hardware (i.e. via the asserted TX-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CFGRrs> {
        TXDMAEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - TX-FIFO flush (whatever I3C is acting as controller/target) This bit can only be written. When the I3C is acting as target, this bit can be used to flush the TX-FIFO on a private read if the controller has early ended the read data (i.e. driven low the T bit) and there is/are remaining data in the TX-FIFO (i.e. I3C_SR.ABT=1 and I3C_SR.XDCNT\\[15:0\\]
&lt; I3C_TGTTDR.TGTTDCNT\\[15:0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn txflush(&mut self) -> TXFLUSH_W<CFGRrs> {
        TXFLUSH_W::new(self, 13)
    }
    #[doc = "Bit 14 - TX-FIFO threshold (whatever I3C is acting as controller/target) This threshold defines, compared to the TX-FIFO level, when the I3C_EVR.TXFNFF flag is set (and consequently if TXDMAEN=1 when is asserted a DMA TX request). TXFNFF is set when 1 byte is to be written in TX-FIFO (i.e. in I3C_TDR). TXFNFF is set when 4 bytes are to be written in TX-FIFO (i.e. in I3C_TDWR)."]
    #[inline(always)]
    #[must_use]
    pub fn txthres(&mut self) -> TXTHRES_W<CFGRrs> {
        TXTHRES_W::new(self, 14)
    }
    #[doc = "Bit 16 - S-FIFO DMA request enable (when I3C is acting as controller) Condition: When RMODE=1 (FIFO is enabled for the status): - Software reads and pops a status word from S-FIFO i.e. reads I3C_SR register after a completed frame (I3C_EVR.FCF=1) or an error (I3C_EVR.ERRF=1). - A status word can be read by the software either via polling on these register flags or via interrupt notification (enabled by I3C_IER.FCIE=1 and I3C_IER.ERRIE=1). - DMA reads and pops status word(s) from S-FIFO i.e. reads I3C_SR register. - Status word(s) are automatically read by the programmed hardware (i.e. via the asserted S-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    #[must_use]
    pub fn sdmaen(&mut self) -> SDMAEN_W<CFGRrs> {
        SDMAEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - S-FIFO flush (when I3C is acting as controller) When I3C is acting as I3C controller, this bit can only be written (and is only used when I3C is acting as controller)."]
    #[inline(always)]
    #[must_use]
    pub fn sflush(&mut self) -> SFLUSH_W<CFGRrs> {
        SFLUSH_W::new(self, 17)
    }
    #[doc = "Bit 18 - S-FIFO enable / status receive mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the enabling the FIFO for the status (S-FIFO) vs the received status from the target on the I3C bus. When I3C is acting as target, this bit must be cleared. - Status register (i.e. I3C_SR) is used without FIFO mechanism. - There is no SCL stretch if a new status register content is not read. - Status register must be read before being lost/overwritten. All message status must be read. There is SCL stretch when there is no more space in the S-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<CFGRrs> {
        RMODE_W::new(self, 18)
    }
    #[doc = "Bit 19 - transmit mode (when I3C is acting as controller) When I3C is acting as I3C controller, this bit is used for the C-FIFO and TX-FIFO management vs the emitted frame on the I3C bus. A frame transfer starts as soon as first control word is present in C-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<CFGRrs> {
        TMODE_W::new(self, 19)
    }
    #[doc = "Bit 20 - C-FIFO DMA request enable (when I3C is acting as controller) When I3C is acting as controller: - Software writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer can be written by software either via polling on the flag I3C_EVR.CFNFF=1 or via interrupt notification (enabled by I3C_IER.CFNFIE=1). - DMA writes and pushes control word(s) into C-FIFO i.e. writes I3C_CR register, as needed for a given frame. - A next control word transfer is automatically written by the programmed hardware (i.e. via the asserted C-FIFO DMA request from the I3C and the programmed DMA channel)."]
    #[inline(always)]
    #[must_use]
    pub fn cdmaen(&mut self) -> CDMAEN_W<CFGRrs> {
        CDMAEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - C-FIFO flush (when I3C is acting as controller) This bit can only be written."]
    #[inline(always)]
    #[must_use]
    pub fn cflush(&mut self) -> CFLUSH_W<CFGRrs> {
        CFLUSH_W::new(self, 21)
    }
    #[doc = "Bit 30 - frame transfer set (a.k.a. software trigger) (when I3C is acting as controller) This bit can only be written. When I3C is acting as I3C controller: Note: If this bit is not set, the other alternative for the software to initiate a frame transfer is to directly write the first control word register (i.e. I3C_CR) while C-FIFO is empty (i.e. I3C_EVR.CFEF=1). Then, if the first written control word is not tagged as a message end (i.e I3C_CR.MEND=0), it causes the hardware to assert the flag I3C_EVR.CFNFF (C-FIFO not full and a next control word is needed)."]
    #[inline(always)]
    #[must_use]
    pub fn tsfset(&mut self) -> TSFSET_W<CFGRrs> {
        TSFSET_W::new(self, 30)
    }
}
#[doc = "I3C configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
