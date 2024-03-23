#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCRrs>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `HD` reader - High drive enable"]
pub type HD_R = crate::BitReader;
#[doc = "Field `HD` writer - High drive enable"]
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - Start of frame interrupt enable"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start of frame interrupt enable"]
pub type SOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDDIE` reader - Update display done interrupt enable"]
pub type UDDIE_R = crate::BitReader;
#[doc = "Field `UDDIE` writer - Update display done interrupt enable"]
pub type UDDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PON` reader - Pulse ON duration"]
pub type PON_R = crate::FieldReader;
#[doc = "Field `PON` writer - Pulse ON duration"]
pub type PON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DEAD` reader - Dead time duration"]
pub type DEAD_R = crate::FieldReader;
#[doc = "Field `DEAD` writer - Dead time duration"]
pub type DEAD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CC` reader - Contrast control"]
pub type CC_R = crate::FieldReader;
#[doc = "Field `CC` writer - Contrast control"]
pub type CC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLINKF` reader - Blink frequency selection"]
pub type BLINKF_R = crate::FieldReader;
#[doc = "Field `BLINKF` writer - Blink frequency selection"]
pub type BLINKF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLINK` reader - Blink mode selection"]
pub type BLINK_R = crate::FieldReader;
#[doc = "Field `BLINK` writer - Blink mode selection"]
pub type BLINK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIV` reader - DIV clock divider"]
pub type DIV_R = crate::FieldReader;
#[doc = "Field `DIV` writer - DIV clock divider"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PS` reader - PS 16-bit prescaler"]
pub type PS_R = crate::FieldReader;
#[doc = "Field `PS` writer - PS 16-bit prescaler"]
pub type PS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Update display done interrupt enable"]
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Pulse ON duration"]
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Contrast control"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - DIV clock divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<FCRrs> {
        HD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<FCRrs> {
        SOFIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Update display done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uddie(&mut self) -> UDDIE_W<FCRrs> {
        UDDIE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Pulse ON duration"]
    #[inline(always)]
    #[must_use]
    pub fn pon(&mut self) -> PON_W<FCRrs> {
        PON_W::new(self, 4)
    }
    #[doc = "Bits 7:9 - Dead time duration"]
    #[inline(always)]
    #[must_use]
    pub fn dead(&mut self) -> DEAD_W<FCRrs> {
        DEAD_W::new(self, 7)
    }
    #[doc = "Bits 10:12 - Contrast control"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<FCRrs> {
        CC_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - Blink frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn blinkf(&mut self) -> BLINKF_W<FCRrs> {
        BLINKF_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Blink mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BLINK_W<FCRrs> {
        BLINK_W::new(self, 16)
    }
    #[doc = "Bits 18:21 - DIV clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<FCRrs> {
        DIV_W::new(self, 18)
    }
    #[doc = "Bits 22:25 - PS 16-bit prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<FCRrs> {
        PS_W::new(self, 22)
    }
}
#[doc = "frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCRrs {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
