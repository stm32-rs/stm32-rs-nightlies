#[doc = "Register `I3C_SR` reader"]
pub type R = crate::R<I3C_SRrs>;
#[doc = "Field `XDCNT` reader - data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message"]
pub type XDCNT_R = crate::FieldReader<u16>;
#[doc = "Field `ABT` reader - a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. I3C_CR.DCNT\\[15:0\\])."]
pub type ABT_R = crate::BitReader;
#[doc = "Field `DIR` reader - message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
pub type DIR_R = crate::BitReader;
#[doc = "Field `MID` reader - message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. I3C_CR) to which the I3C_SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. I3C_CR) over I3C bus. This field is reset for every new frame start."]
pub type MID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - data counter - When the I3C is acting as controller: number of targets detected on the bus - When the I3C is acting as target: number of transmitted bytes - Whatever the I3C is acting as controller or target: number of data bytes read from or transmitted on the I3C bus during the MID\\[7:0\\]
message"]
    #[inline(always)]
    pub fn xdcnt(&self) -> XDCNT_R {
        XDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 17 - a private read message is completed/aborted prematurely by the target (when the I3C is acting as controller) When the I3C is acting as controller, this bit indicates if the private read data which is transmitted by the target early terminates (i.e. the target drives T bit low earlier vs what does expect the controller in terms of programmed number of read data bytes i.e. I3C_CR.DCNT\\[15:0\\])."]
    #[inline(always)]
    pub fn abt(&self) -> ABT_R {
        ABT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - message direction Whatever the I3C is acting as controller or target, this bit indicates the direction of the related message on the I3C bus Note: ENTDAA CCC is considered as a write command."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:31 - message identifier/counter of a given frame (when the I3C is acting as controller) When the I3C is acting as controller, this field identifies the control word message (i.e. I3C_CR) to which the I3C_SR status register refers. First message of a frame is identified with MID\\[7:0\\]=0. This field is incremented (by hardware) on the completion of a new message control word (i.e. I3C_CR) over I3C bus. This field is reset for every new frame start."]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "I3C status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_SRrs;
impl crate::RegisterSpec for I3C_SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_sr::R`](R) reader structure"]
impl crate::Readable for I3C_SRrs {}
#[doc = "`reset()` method sets I3C_SR to value 0"]
impl crate::Resettable for I3C_SRrs {
    const RESET_VALUE: u32 = 0;
}
