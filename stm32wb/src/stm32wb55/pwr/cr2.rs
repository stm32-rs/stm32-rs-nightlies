#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PLS_R = crate::FieldReader;
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_R = crate::BitReader;
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_R = crate::BitReader;
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USV` reader - VDDUSB USB supply valid"]
pub type USV_R = crate::BitReader;
#[doc = "Field `USV` writer - VDDUSB USB supply valid"]
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR2rs> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<CR2rs> {
        PLS_W::new(self, 1)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme1(&mut self) -> PVME1_W<CR2rs> {
        PVME1_W::new(self, 4)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> PVME3_W<CR2rs> {
        PVME3_W::new(self, 6)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<CR2rs> {
        USV_W::new(self, 10)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
