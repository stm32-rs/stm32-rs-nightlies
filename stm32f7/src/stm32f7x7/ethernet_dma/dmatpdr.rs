#[doc = "Register `DMATPDR` reader"]
pub type R = crate::R<DMATPDRrs>;
#[doc = "Register `DMATPDR` writer"]
pub type W = crate::W<DMATPDRrs>;
#[doc = "Transmit poll demand\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TPD {
    #[doc = "0: Poll the transmit descriptor list"]
    Poll = 0,
}
impl From<TPD> for u32 {
    #[inline(always)]
    fn from(variant: TPD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TPD {
    type Ux = u32;
}
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TPD_R = crate::FieldReader<TPD>;
impl TPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TPD> {
        match self.bits {
            0 => Some(TPD::Poll),
            _ => None,
        }
    }
    #[doc = "Poll the transmit descriptor list"]
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == TPD::Poll
    }
}
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, TPD>;
impl<'a, REG> TPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Poll the transmit descriptor list"]
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(TPD::Poll)
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<DMATPDRrs> {
        TPD_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPDRrs;
impl crate::RegisterSpec for DMATPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpdr::R`](R) reader structure"]
impl crate::Readable for DMATPDRrs {}
#[doc = "`write(|w| ..)` method takes [`dmatpdr::W`](W) writer structure"]
impl crate::Writable for DMATPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATPDR to value 0"]
impl crate::Resettable for DMATPDRrs {
    const RESET_VALUE: u32 = 0;
}
