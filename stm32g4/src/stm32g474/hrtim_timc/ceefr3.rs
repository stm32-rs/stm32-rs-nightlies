///Register `CEEFR3` reader
pub type R = crate::R<CEEFR3rs>;
///Register `CEEFR3` writer
pub type W = crate::W<CEEFR3rs>;
///Field `EEVACE` reader - External Event A Counter Enable
pub type EEVACE_R = crate::BitReader;
///Field `EEVACE` writer - External Event A Counter Enable
pub type EEVACE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEVACRES` reader - External Event A Counter Reset
pub type EEVACRES_R = crate::BitReader;
///Field `EEVACRES` writer - External Event A Counter Reset
pub type EEVACRES_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEVARSTM` reader - External Event A Reset Mode
pub type EEVARSTM_R = crate::BitReader;
///Field `EEVARSTM` writer - External Event A Reset Mode
pub type EEVARSTM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EEVASEL` reader - External Event A Selection
pub type EEVASEL_R = crate::FieldReader;
///Field `EEVASEL` writer - External Event A Selection
pub type EEVASEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `EEVACNT` reader - External Event A counter
pub type EEVACNT_R = crate::FieldReader;
///Field `EEVACNT` writer - External Event A counter
pub type EEVACNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - External Event A Counter Enable
    #[inline(always)]
    pub fn eevace(&self) -> EEVACE_R {
        EEVACE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - External Event A Counter Reset
    #[inline(always)]
    pub fn eevacres(&self) -> EEVACRES_R {
        EEVACRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External Event A Reset Mode
    #[inline(always)]
    pub fn eevarstm(&self) -> EEVARSTM_R {
        EEVARSTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - External Event A Selection
    #[inline(always)]
    pub fn eevasel(&self) -> EEVASEL_R {
        EEVASEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:13 - External Event A counter
    #[inline(always)]
    pub fn eevacnt(&self) -> EEVACNT_R {
        EEVACNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CEEFR3")
            .field("eevacnt", &self.eevacnt())
            .field("eevasel", &self.eevasel())
            .field("eevarstm", &self.eevarstm())
            .field("eevacres", &self.eevacres())
            .field("eevace", &self.eevace())
            .finish()
    }
}
impl W {
    ///Bit 0 - External Event A Counter Enable
    #[inline(always)]
    #[must_use]
    pub fn eevace(&mut self) -> EEVACE_W<CEEFR3rs> {
        EEVACE_W::new(self, 0)
    }
    ///Bit 1 - External Event A Counter Reset
    #[inline(always)]
    #[must_use]
    pub fn eevacres(&mut self) -> EEVACRES_W<CEEFR3rs> {
        EEVACRES_W::new(self, 1)
    }
    ///Bit 2 - External Event A Reset Mode
    #[inline(always)]
    #[must_use]
    pub fn eevarstm(&mut self) -> EEVARSTM_W<CEEFR3rs> {
        EEVARSTM_W::new(self, 2)
    }
    ///Bits 4:7 - External Event A Selection
    #[inline(always)]
    #[must_use]
    pub fn eevasel(&mut self) -> EEVASEL_W<CEEFR3rs> {
        EEVASEL_W::new(self, 4)
    }
    ///Bits 8:13 - External Event A counter
    #[inline(always)]
    #[must_use]
    pub fn eevacnt(&mut self) -> EEVACNT_W<CEEFR3rs> {
        EEVACNT_W::new(self, 8)
    }
}
/**HRTIM Timerx External Event Filtering Register 3

You can [`read`](crate::Reg::read) this register and get [`ceefr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceefr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIMC:CEEFR3)*/
pub struct CEEFR3rs;
impl crate::RegisterSpec for CEEFR3rs {
    type Ux = u32;
}
///`read()` method returns [`ceefr3::R`](R) reader structure
impl crate::Readable for CEEFR3rs {}
///`write(|w| ..)` method takes [`ceefr3::W`](W) writer structure
impl crate::Writable for CEEFR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CEEFR3 to value 0
impl crate::Resettable for CEEFR3rs {
    const RESET_VALUE: u32 = 0;
}
