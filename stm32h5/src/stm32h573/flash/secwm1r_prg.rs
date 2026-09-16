///Register `SECWM1R_PRG` reader
pub type R = crate::R<SECWM1R_PRGrs>;
///Register `SECWM1R_PRG` writer
pub type W = crate::W<SECWM1R_PRGrs>;
///Field `SECWM1_STRT` reader - Bank1 security WM area 1 start sector
pub type SECWM1_STRT_R = crate::FieldReader;
///Field `SECWM1_STRT` writer - Bank1 security WM area 1 start sector
pub type SECWM1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SECWM1_END` reader - Bank1 security WM area 1 end sector
pub type SECWM1_END_R = crate::FieldReader;
///Field `SECWM1_END` writer - Bank1 security WM area 1 end sector
pub type SECWM1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Bank1 security WM area 1 start sector
    #[inline(always)]
    pub fn secwm1_strt(&self) -> SECWM1_STRT_R {
        SECWM1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank1 security WM area 1 end sector
    #[inline(always)]
    pub fn secwm1_end(&self) -> SECWM1_END_R {
        SECWM1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM1R_PRG")
            .field("secwm1_strt", &self.secwm1_strt())
            .field("secwm1_end", &self.secwm1_end())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Bank1 security WM area 1 start sector
    #[inline(always)]
    pub fn secwm1_strt(&mut self) -> SECWM1_STRT_W<'_, SECWM1R_PRGrs> {
        SECWM1_STRT_W::new(self, 0)
    }
    ///Bits 16:22 - Bank1 security WM area 1 end sector
    #[inline(always)]
    pub fn secwm1_end(&mut self) -> SECWM1_END_W<'_, SECWM1R_PRGrs> {
        SECWM1_END_W::new(self, 16)
    }
}
/**FLASH security watermark for Bank 1

You can [`read`](crate::Reg::read) this register and get [`secwm1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:SECWM1R_PRG)*/
pub struct SECWM1R_PRGrs;
impl crate::RegisterSpec for SECWM1R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`secwm1r_prg::R`](R) reader structure
impl crate::Readable for SECWM1R_PRGrs {}
///`write(|w| ..)` method takes [`secwm1r_prg::W`](W) writer structure
impl crate::Writable for SECWM1R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM1R_PRG to value 0
impl crate::Resettable for SECWM1R_PRGrs {}
