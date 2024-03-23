#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `MUDIS` reader - Master Update Disable"]
pub type MUDIS_R = crate::BitReader;
#[doc = "Field `MUDIS` writer - Master Update Disable"]
pub type MUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAUDIS` reader - Timer A Update Disable"]
pub type TAUDIS_R = crate::BitReader;
#[doc = "Field `TAUDIS` writer - Timer A Update Disable"]
pub type TAUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUDIS` reader - Timer B Update Disable"]
pub type TBUDIS_R = crate::BitReader;
#[doc = "Field `TBUDIS` writer - Timer B Update Disable"]
pub type TBUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCUDIS` reader - Timer C Update Disable"]
pub type TCUDIS_R = crate::BitReader;
#[doc = "Field `TCUDIS` writer - Timer C Update Disable"]
pub type TCUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDUDIS` reader - Timer D Update Disable"]
pub type TDUDIS_R = crate::BitReader;
#[doc = "Field `TDUDIS` writer - Timer D Update Disable"]
pub type TDUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEUDIS` reader - Timer E Update Disable"]
pub type TEUDIS_R = crate::BitReader;
#[doc = "Field `TEUDIS` writer - Timer E Update Disable"]
pub type TEUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFUDIS` reader - Timer f Update Disable"]
pub type TFUDIS_R = crate::BitReader;
#[doc = "Field `TFUDIS` writer - Timer f Update Disable"]
pub type TFUDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD1USRC` reader - ADC Trigger 1 Update Source"]
pub type AD1USRC_R = crate::FieldReader;
#[doc = "Field `AD1USRC` writer - ADC Trigger 1 Update Source"]
pub type AD1USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD2USRC` reader - ADC Trigger 2 Update Source"]
pub type AD2USRC_R = crate::FieldReader;
#[doc = "Field `AD2USRC` writer - ADC Trigger 2 Update Source"]
pub type AD2USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD3USRC` reader - ADC Trigger 3 Update Source"]
pub type AD3USRC_R = crate::FieldReader;
#[doc = "Field `AD3USRC` writer - ADC Trigger 3 Update Source"]
pub type AD3USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AD4USRC` reader - ADC Trigger 4 Update Source"]
pub type AD4USRC_R = crate::FieldReader;
#[doc = "Field `AD4USRC` writer - ADC Trigger 4 Update Source"]
pub type AD4USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&self) -> TAUDIS_R {
        TAUDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&self) -> TBUDIS_R {
        TBUDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&self) -> TCUDIS_R {
        TCUDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&self) -> TDUDIS_R {
        TDUDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&self) -> TEUDIS_R {
        TEUDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    pub fn tfudis(&self) -> TFUDIS_R {
        TFUDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&self) -> AD1USRC_R {
        AD1USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&self) -> AD2USRC_R {
        AD2USRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&self) -> AD3USRC_R {
        AD3USRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&self) -> AD4USRC_R {
        AD4USRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mudis(&mut self) -> MUDIS_W<CR1rs> {
        MUDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn taudis(&mut self) -> TAUDIS_W<CR1rs> {
        TAUDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tbudis(&mut self) -> TBUDIS_W<CR1rs> {
        TBUDIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tcudis(&mut self) -> TCUDIS_W<CR1rs> {
        TCUDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdudis(&mut self) -> TDUDIS_W<CR1rs> {
        TDUDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn teudis(&mut self) -> TEUDIS_W<CR1rs> {
        TEUDIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer f Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tfudis(&mut self) -> TFUDIS_W<CR1rs> {
        TFUDIS_W::new(self, 6)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad1usrc(&mut self) -> AD1USRC_W<CR1rs> {
        AD1USRC_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad2usrc(&mut self) -> AD2USRC_W<CR1rs> {
        AD2USRC_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad3usrc(&mut self) -> AD3USRC_W<CR1rs> {
        AD3USRC_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad4usrc(&mut self) -> AD4USRC_W<CR1rs> {
        AD4USRC_W::new(self, 25)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
