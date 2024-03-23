#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `AWDCH` reader - Analog watchdog channel select bits"]
pub type AWDCH_R = crate::FieldReader;
#[doc = "Field `AWDCH` writer - Analog watchdog channel select bits"]
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EOCIE` reader - Interrupt enable for EOC"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - Interrupt enable for EOC"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader;
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - Interrupt enable for injected channels"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - Interrupt enable for injected channels"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN` reader - Scan mode"]
pub type SCAN_R = crate::BitReader;
#[doc = "Field `SCAN` writer - Scan mode"]
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDSGL` reader - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_R = crate::BitReader;
#[doc = "Field `AWDSGL` writer - Enable the watchdog on a single channel in scan mode"]
pub type AWDSGL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - Automatic injected group conversion"]
pub type JAUTO_R = crate::BitReader;
#[doc = "Field `JAUTO` writer - Automatic injected group conversion"]
pub type JAUTO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - Discontinuous mode on regular channels"]
pub type DISCEN_R = crate::BitReader;
#[doc = "Field `DISCEN` writer - Discontinuous mode on regular channels"]
pub type DISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JDISCEN` reader - Discontinuous mode on injected channels"]
pub type JDISCEN_R = crate::BitReader;
#[doc = "Field `JDISCEN` writer - Discontinuous mode on injected channels"]
pub type JDISCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - Discontinuous mode channel count"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - Discontinuous mode channel count"]
pub type DISCNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PDD` reader - Power down during the delay phase"]
pub type PDD_R = crate::BitReader;
#[doc = "Field `PDD` writer - Power down during the delay phase"]
pub type PDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDI` reader - Power down during the idle phase"]
pub type PDI_R = crate::BitReader;
#[doc = "Field `PDI` writer - Power down during the idle phase"]
pub type PDI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWDEN` reader - Analog watchdog enable on injected channels"]
pub type JAWDEN_R = crate::BitReader;
#[doc = "Field `JAWDEN` writer - Analog watchdog enable on injected channels"]
pub type JAWDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDEN` reader - Analog watchdog enable on regular channels"]
pub type AWDEN_R = crate::BitReader;
#[doc = "Field `AWDEN` writer - Analog watchdog enable on regular channels"]
pub type AWDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - Resolution"]
pub type RES_R = crate::FieldReader;
#[doc = "Field `RES` writer - Resolution"]
pub type RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRIE` reader - Overrun interrupt enable"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - Overrun interrupt enable"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Power down during the delay phase"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Power down during the idle phase"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    pub fn jawden(&self) -> JAWDEN_R {
        JAWDEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog watchdog channel select bits"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<CR1rs> {
        AWDCH_W::new(self, 0)
    }
    #[doc = "Bit 5 - Interrupt enable for EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CR1rs> {
        EOCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<CR1rs> {
        AWDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<CR1rs> {
        JEOCIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<CR1rs> {
        SCAN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable the watchdog on a single channel in scan mode"]
    #[inline(always)]
    #[must_use]
    pub fn awdsgl(&mut self) -> AWDSGL_W<CR1rs> {
        AWDSGL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Automatic injected group conversion"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<CR1rs> {
        JAUTO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Discontinuous mode on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<CR1rs> {
        DISCEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Discontinuous mode on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<CR1rs> {
        JDISCEN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Discontinuous mode channel count"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<CR1rs> {
        DISCNUM_W::new(self, 13)
    }
    #[doc = "Bit 16 - Power down during the delay phase"]
    #[inline(always)]
    #[must_use]
    pub fn pdd(&mut self) -> PDD_W<CR1rs> {
        PDD_W::new(self, 16)
    }
    #[doc = "Bit 17 - Power down during the idle phase"]
    #[inline(always)]
    #[must_use]
    pub fn pdi(&mut self) -> PDI_W<CR1rs> {
        PDI_W::new(self, 17)
    }
    #[doc = "Bit 22 - Analog watchdog enable on injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jawden(&mut self) -> JAWDEN_W<CR1rs> {
        JAWDEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Analog watchdog enable on regular channels"]
    #[inline(always)]
    #[must_use]
    pub fn awden(&mut self) -> AWDEN_W<CR1rs> {
        AWDEN_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<CR1rs> {
        RES_W::new(self, 24)
    }
    #[doc = "Bit 26 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<CR1rs> {
        OVRIE_W::new(self, 26)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
