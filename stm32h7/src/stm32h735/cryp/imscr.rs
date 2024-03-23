#[doc = "Register `IMSCR` reader"]
pub type R = crate::R<IMSCRrs>;
#[doc = "Register `IMSCR` writer"]
pub type W = crate::W<IMSCRrs>;
#[doc = "Field `INIM` reader - Input FIFO service interrupt mask"]
pub type INIM_R = crate::BitReader;
#[doc = "Field `INIM` writer - Input FIFO service interrupt mask"]
pub type INIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTIM` reader - Output FIFO service interrupt mask"]
pub type OUTIM_R = crate::BitReader;
#[doc = "Field `OUTIM` writer - Output FIFO service interrupt mask"]
pub type OUTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input FIFO service interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn inim(&mut self) -> INIM_W<IMSCRrs> {
        INIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Output FIFO service interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn outim(&mut self) -> OUTIM_W<IMSCRrs> {
        OUTIM_W::new(self, 1)
    }
}
#[doc = "interrupt mask set/clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMSCRrs;
impl crate::RegisterSpec for IMSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imscr::R`](R) reader structure"]
impl crate::Readable for IMSCRrs {}
#[doc = "`write(|w| ..)` method takes [`imscr::W`](W) writer structure"]
impl crate::Writable for IMSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSCR to value 0"]
impl crate::Resettable for IMSCRrs {
    const RESET_VALUE: u32 = 0;
}
