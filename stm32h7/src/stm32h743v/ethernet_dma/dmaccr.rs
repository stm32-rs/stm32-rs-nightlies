#[doc = "Register `DMACCR` reader"]
pub type R = crate::R<DMACCRrs>;
#[doc = "Register `DMACCR` writer"]
pub type W = crate::W<DMACCRrs>;
#[doc = "Field `MSS` reader - Maximum Segment Size"]
pub type MSS_R = crate::FieldReader<u16>;
#[doc = "Field `MSS` writer - Maximum Segment Size"]
pub type MSS_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PBLX8` reader - 8xPBL mode"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLX8` writer - 8xPBL mode"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Segment Size"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<DMACCRrs> {
        MSS_W::new(self, 0)
    }
    #[doc = "Bit 16 - 8xPBL mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<DMACCRrs> {
        PBLX8_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Descriptor Skip Length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMACCRrs> {
        DSL_W::new(self, 18)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACCRrs;
impl crate::RegisterSpec for DMACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccr::R`](R) reader structure"]
impl crate::Readable for DMACCRrs {}
#[doc = "`write(|w| ..)` method takes [`dmaccr::W`](W) writer structure"]
impl crate::Writable for DMACCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACCR to value 0"]
impl crate::Resettable for DMACCRrs {
    const RESET_VALUE: u32 = 0;
}
