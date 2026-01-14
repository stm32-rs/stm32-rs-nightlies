///Register `CR0` reader
pub type R = crate::R<CR0rs>;
///Register `CR0` writer
pub type W = crate::W<CR0rs>;
///Field `TMEAS` reader - Measurement Time
pub type TMEAS_R = crate::FieldReader<u16>;
///Field `TMEAS` writer - Measurement Time
pub type TMEAS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `TCAP` reader - Capture Time
pub type TCAP_R = crate::FieldReader;
///Field `TCAP` writer - Capture Time
pub type TCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `TICAP` reader - Inter Capture Time
pub type TICAP_R = crate::FieldReader;
///Field `TICAP` writer - Inter Capture Time
pub type TICAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:13 - Measurement Time
    #[inline(always)]
    pub fn tmeas(&self) -> TMEAS_R {
        TMEAS_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:21 - Capture Time
    #[inline(always)]
    pub fn tcap(&self) -> TCAP_R {
        TCAP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:26 - Inter Capture Time
    #[inline(always)]
    pub fn ticap(&self) -> TICAP_R {
        TICAP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0")
            .field("tmeas", &self.tmeas())
            .field("tcap", &self.tcap())
            .field("ticap", &self.ticap())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Measurement Time
    #[inline(always)]
    pub fn tmeas(&mut self) -> TMEAS_W<'_, CR0rs> {
        TMEAS_W::new(self, 0)
    }
    ///Bits 16:21 - Capture Time
    #[inline(always)]
    pub fn tcap(&mut self) -> TCAP_W<'_, CR0rs> {
        TCAP_W::new(self, 16)
    }
    ///Bits 24:26 - Inter Capture Time
    #[inline(always)]
    pub fn ticap(&mut self) -> TICAP_W<'_, CR0rs> {
        TICAP_W::new(self, 24)
    }
}
/**LCSC_CR0 register

You can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LCSC:CR0)*/
pub struct CR0rs;
impl crate::RegisterSpec for CR0rs {
    type Ux = u32;
}
///`read()` method returns [`cr0::R`](R) reader structure
impl crate::Readable for CR0rs {}
///`write(|w| ..)` method takes [`cr0::W`](W) writer structure
impl crate::Writable for CR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR0 to value 0x000b_005c
impl crate::Resettable for CR0rs {
    const RESET_VALUE: u32 = 0x000b_005c;
}
