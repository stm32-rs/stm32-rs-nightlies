///Register `FMR` reader
pub type R = crate::R<FMRrs>;
///Register `FMR` writer
pub type W = crate::W<FMRrs>;
///Field `FINIT` reader - FINIT
pub type FINIT_R = crate::BitReader;
///Field `FINIT` writer - FINIT
pub type FINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN2SB` reader - CAN2SB
pub type CAN2SB_R = crate::FieldReader;
///Field `CAN2SB` writer - CAN2SB
pub type CAN2SB_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - FINIT
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:13 - CAN2SB
    #[inline(always)]
    pub fn can2sb(&self) -> CAN2SB_R {
        CAN2SB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMR")
            .field("can2sb", &self.can2sb())
            .field("finit", &self.finit())
            .finish()
    }
}
impl W {
    ///Bit 0 - FINIT
    #[inline(always)]
    pub fn finit(&mut self) -> FINIT_W<'_, FMRrs> {
        FINIT_W::new(self, 0)
    }
    ///Bits 8:13 - CAN2SB
    #[inline(always)]
    pub fn can2sb(&mut self) -> CAN2SB_W<'_, FMRrs> {
        CAN2SB_W::new(self, 8)
    }
}
/**filter master register

You can [`read`](crate::Reg::read) this register and get [`fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#CAN1:FMR)*/
pub struct FMRrs;
impl crate::RegisterSpec for FMRrs {
    type Ux = u32;
}
///`read()` method returns [`fmr::R`](R) reader structure
impl crate::Readable for FMRrs {}
///`write(|w| ..)` method takes [`fmr::W`](W) writer structure
impl crate::Writable for FMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMR to value 0x2a1c_0e01
impl crate::Resettable for FMRrs {
    const RESET_VALUE: u32 = 0x2a1c_0e01;
}
