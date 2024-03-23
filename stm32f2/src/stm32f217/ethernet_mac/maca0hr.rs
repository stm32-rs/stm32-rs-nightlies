#[doc = "Register `MACA0HR` reader"]
pub type R = crate::R<MACA0HRrs>;
#[doc = "Register `MACA0HR` writer"]
pub type W = crate::W<MACA0HRrs>;
#[doc = "Field `MACA0H` reader - MAC address0 high"]
pub type MACA0H_R = crate::FieldReader<u16>;
#[doc = "Field `MACA0H` writer - MAC address0 high"]
pub type MACA0H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MO` reader - MO"]
pub type MO_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn maca0h(&self) -> MACA0H_R {
        MACA0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - MO"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    #[must_use]
    pub fn maca0h(&mut self) -> MACA0H_W<MACA0HRrs> {
        MACA0H_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA0HRrs;
impl crate::RegisterSpec for MACA0HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0hr::R`](R) reader structure"]
impl crate::Readable for MACA0HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca0hr::W`](W) writer structure"]
impl crate::Writable for MACA0HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0HR to value 0x0010_ffff"]
impl crate::Resettable for MACA0HRrs {
    const RESET_VALUE: u32 = 0x0010_ffff;
}
