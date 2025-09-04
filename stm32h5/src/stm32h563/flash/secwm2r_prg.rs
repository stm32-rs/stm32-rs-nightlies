///Register `SECWM2R_PRG` reader
pub type R = crate::R<SECWM2R_PRGrs>;
///Register `SECWM2R_PRG` writer
pub type W = crate::W<SECWM2R_PRGrs>;
///Field `SECWM_STRT2` reader - Bank 2 security WM area start sector
pub type SECWM_STRT2_R = crate::FieldReader;
///Field `SECWM_STRT2` writer - Bank 2 security WM area start sector
pub type SECWM_STRT2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SECWM_END2` reader - Bank 2 security WM area end sector
pub type SECWM_END2_R = crate::FieldReader;
///Field `SECWM_END2` writer - Bank 2 security WM area end sector
pub type SECWM_END2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Bank 2 security WM area start sector
    #[inline(always)]
    pub fn secwm_strt2(&self) -> SECWM_STRT2_R {
        SECWM_STRT2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank 2 security WM area end sector
    #[inline(always)]
    pub fn secwm_end2(&self) -> SECWM_END2_R {
        SECWM_END2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECWM2R_PRG")
            .field("secwm_strt2", &self.secwm_strt2())
            .field("secwm_end2", &self.secwm_end2())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Bank 2 security WM area start sector
    #[inline(always)]
    pub fn secwm_strt2(&mut self) -> SECWM_STRT2_W<SECWM2R_PRGrs> {
        SECWM_STRT2_W::new(self, 0)
    }
    ///Bits 16:22 - Bank 2 security WM area end sector
    #[inline(always)]
    pub fn secwm_end2(&mut self) -> SECWM_END2_W<SECWM2R_PRGrs> {
        SECWM_END2_W::new(self, 16)
    }
}
/**FLASH security watermark for Bank 2

You can [`read`](crate::Reg::read) this register and get [`secwm2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secwm2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:SECWM2R_PRG)*/
pub struct SECWM2R_PRGrs;
impl crate::RegisterSpec for SECWM2R_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`secwm2r_prg::R`](R) reader structure
impl crate::Readable for SECWM2R_PRGrs {}
///`write(|w| ..)` method takes [`secwm2r_prg::W`](W) writer structure
impl crate::Writable for SECWM2R_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECWM2R_PRG to value 0
impl crate::Resettable for SECWM2R_PRGrs {}
