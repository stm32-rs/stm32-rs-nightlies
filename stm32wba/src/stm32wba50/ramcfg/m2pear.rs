///Register `M2PEAR` reader
pub type R = crate::R<M2PEARrs>;
///Field `PEA` reader - Parity error SRAM word aligned address offset.PEA\[1:0\] read 0b00. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with the SRAM word aligned address offset corresponding to the parity error.
pub type PEA_R = crate::FieldReader<u16>;
///Field `ID` reader - Parity error AHB bus master ID. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with: Others: reserved
pub type ID_R = crate::FieldReader;
///Field `BYTE` reader - Byte parity error flag. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with: 1xxx: parity error detected on fourth byte in word aligned address x1xx: parity error detected on third byte in word aligned address xx1x: parity error detected on second byte in word aligned address xxx1: parity error detected on first byte in word aligned address
pub type BYTE_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Parity error SRAM word aligned address offset.PEA\[1:0\] read 0b00. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with the SRAM word aligned address offset corresponding to the parity error.
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:27 - Parity error AHB bus master ID. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with: Others: reserved
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Byte parity error flag. When ALE bit is set in RAMCFG_M2CR register, this field is updated when PED and CPED are zero and a new parity error is detected, with: 1xxx: parity error detected on fourth byte in word aligned address x1xx: parity error detected on third byte in word aligned address xx1x: parity error detected on second byte in word aligned address xxx1: parity error detected on first byte in word aligned address
    #[inline(always)]
    pub fn byte(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2PEAR")
            .field("pea", &self.pea())
            .field("id", &self.id())
            .field("byte", &self.byte())
            .finish()
    }
}
/**RAMCFG SRAM2 parity error address register

You can [`read`](crate::Reg::read) this register and get [`m2pear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M2PEAR)*/
pub struct M2PEARrs;
impl crate::RegisterSpec for M2PEARrs {
    type Ux = u32;
}
///`read()` method returns [`m2pear::R`](R) reader structure
impl crate::Readable for M2PEARrs {}
///`reset()` method sets M2PEAR to value 0
impl crate::Resettable for M2PEARrs {}
