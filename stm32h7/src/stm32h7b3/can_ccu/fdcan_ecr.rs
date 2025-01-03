///Register `FDCAN_ECR` reader
pub type R = crate::R<FDCAN_ECRrs>;
///Register `FDCAN_ECR` writer
pub type W = crate::W<FDCAN_ECRrs>;
///Field `TEC` reader - Transmit Error Counter
pub type TEC_R = crate::FieldReader;
///Field `TEC` writer - Transmit Error Counter
pub type TEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TREC` reader - Receive Error Counter
pub type TREC_R = crate::FieldReader;
///Field `TREC` writer - Receive Error Counter
pub type TREC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `RP` reader - Receive Error Passive
pub type RP_R = crate::BitReader;
///Field `RP` writer - Receive Error Passive
pub type RP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEL` reader - AN Error Logging
pub type CEL_R = crate::FieldReader;
///Field `CEL` writer - AN Error Logging
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Transmit Error Counter
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Receive Error Counter
    #[inline(always)]
    pub fn trec(&self) -> TREC_R {
        TREC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Receive Error Passive
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ECR")
            .field("cel", &self.cel())
            .field("rp", &self.rp())
            .field("trec", &self.trec())
            .field("tec", &self.tec())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Transmit Error Counter
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W<FDCAN_ECRrs> {
        TEC_W::new(self, 0)
    }
    ///Bits 8:14 - Receive Error Counter
    #[inline(always)]
    pub fn trec(&mut self) -> TREC_W<FDCAN_ECRrs> {
        TREC_W::new(self, 8)
    }
    ///Bit 15 - Receive Error Passive
    #[inline(always)]
    pub fn rp(&mut self) -> RP_W<FDCAN_ECRrs> {
        RP_W::new(self, 15)
    }
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    pub fn cel(&mut self) -> CEL_W<FDCAN_ECRrs> {
        CEL_W::new(self, 16)
    }
}
/**FDCAN Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CAN_CCU:FDCAN_ECR)*/
pub struct FDCAN_ECRrs;
impl crate::RegisterSpec for FDCAN_ECRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ecr::R`](R) reader structure
impl crate::Readable for FDCAN_ECRrs {}
///`write(|w| ..)` method takes [`fdcan_ecr::W`](W) writer structure
impl crate::Writable for FDCAN_ECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_ECR to value 0
impl crate::Resettable for FDCAN_ECRrs {
    const RESET_VALUE: u32 = 0;
}
