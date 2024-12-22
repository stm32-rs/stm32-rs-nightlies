///Register `CAN_TTGTP` reader
pub type R = crate::R<CAN_TTGTPrs>;
///Register `CAN_TTGTP` writer
pub type W = crate::W<CAN_TTGTPrs>;
///Field `NCL` reader - Time Preset
pub type NCL_R = crate::FieldReader<u16>;
///Field `NCL` writer - Time Preset
pub type NCL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `CTP` reader - Cycle Time Target Phase
pub type CTP_R = crate::FieldReader<u16>;
///Field `CTP` writer - Cycle Time Target Phase
pub type CTP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Time Preset
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Cycle Time Target Phase
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_TTGTP")
            .field("ncl", &self.ncl())
            .field("ctp", &self.ctp())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Time Preset
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W<CAN_TTGTPrs> {
        NCL_W::new(self, 0)
    }
    ///Bits 16:31 - Cycle Time Target Phase
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W<CAN_TTGTPrs> {
        CTP_W::new(self, 16)
    }
}
/**FDCAN TT Global Time Preset Register

You can [`read`](crate::Reg::read) this register and get [`can_ttgtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ttgtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:CAN_TTGTP)*/
pub struct CAN_TTGTPrs;
impl crate::RegisterSpec for CAN_TTGTPrs {
    type Ux = u32;
}
///`read()` method returns [`can_ttgtp::R`](R) reader structure
impl crate::Readable for CAN_TTGTPrs {}
///`write(|w| ..)` method takes [`can_ttgtp::W`](W) writer structure
impl crate::Writable for CAN_TTGTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAN_TTGTP to value 0
impl crate::Resettable for CAN_TTGTPrs {
    const RESET_VALUE: u32 = 0;
}
