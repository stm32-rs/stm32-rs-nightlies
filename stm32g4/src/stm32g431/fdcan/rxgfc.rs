#[doc = "Register `RXGFC` reader"]
pub type R = crate::R<RXGFCrs>;
#[doc = "Register `RXGFC` writer"]
pub type W = crate::W<RXGFCrs>;
#[doc = "Field `RRFE` reader - RRFE"]
pub type RRFE_R = crate::BitReader;
#[doc = "Field `RRFE` writer - RRFE"]
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRFS` reader - RRFS"]
pub type RRFS_R = crate::BitReader;
#[doc = "Field `RRFS` writer - RRFS"]
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANFE` writer - ANFE"]
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANFS` writer - ANFS"]
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `F1OM` reader - FIFO 1 operation mode"]
pub type F1OM_R = crate::BitReader;
#[doc = "Field `F1OM` writer - FIFO 1 operation mode"]
pub type F1OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0OM` reader - FIFO 0 operation mode"]
pub type F0OM_R = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 operation mode"]
pub type F0OM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSS` reader - List size standard"]
pub type LSS_R = crate::FieldReader;
#[doc = "Field `LSS` writer - List size standard"]
pub type LSS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LSE` reader - List size extended"]
pub type LSE_R = crate::FieldReader;
#[doc = "Field `LSE` writer - List size extended"]
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    #[must_use]
    pub fn rrfe(&mut self) -> RRFE_W<RXGFCrs> {
        RRFE_W::new(self, 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    #[must_use]
    pub fn rrfs(&mut self) -> RRFS_W<RXGFCrs> {
        RRFS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    #[must_use]
    pub fn anfe(&mut self) -> ANFE_W<RXGFCrs> {
        ANFE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    #[must_use]
    pub fn anfs(&mut self) -> ANFS_W<RXGFCrs> {
        ANFS_W::new(self, 4)
    }
    #[doc = "Bit 8 - FIFO 1 operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn f1om(&mut self) -> F1OM_W<RXGFCrs> {
        F1OM_W::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO 0 operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn f0om(&mut self) -> F0OM_W<RXGFCrs> {
        F0OM_W::new(self, 9)
    }
    #[doc = "Bits 16:20 - List size standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss(&mut self) -> LSS_W<RXGFCrs> {
        LSS_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - List size extended"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<RXGFCrs> {
        LSE_W::new(self, 24)
    }
}
#[doc = "Global settings for Message ID filtering. The Global Filter Configuration controls the filter path for standard and extended messages as described in Figure706: Standard Message ID filter path and Figure707: Extended Message ID filter path.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxgfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxgfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXGFCrs;
impl crate::RegisterSpec for RXGFCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxgfc::R`](R) reader structure"]
impl crate::Readable for RXGFCrs {}
#[doc = "`write(|w| ..)` method takes [`rxgfc::W`](W) writer structure"]
impl crate::Writable for RXGFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXGFC to value 0"]
impl crate::Resettable for RXGFCrs {
    const RESET_VALUE: u32 = 0;
}
