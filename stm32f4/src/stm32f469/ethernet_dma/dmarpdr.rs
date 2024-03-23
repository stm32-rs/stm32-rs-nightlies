#[doc = "Register `DMARPDR` reader"]
pub type R = crate::R<DMARPDRrs>;
#[doc = "Register `DMARPDR` writer"]
pub type W = crate::W<DMARPDRrs>;
#[doc = "Receive poll demand\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RPD {
    #[doc = "0: Poll the receive descriptor list"]
    Poll = 0,
}
impl From<RPD> for u32 {
    #[inline(always)]
    fn from(variant: RPD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPD {
    type Ux = u32;
}
#[doc = "Field `RPD` reader - Receive poll demand"]
pub type RPD_R = crate::FieldReader<RPD>;
impl RPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPD> {
        match self.bits {
            0 => Some(RPD::Poll),
            _ => None,
        }
    }
    #[doc = "Poll the receive descriptor list"]
    #[inline(always)]
    pub fn is_poll(&self) -> bool {
        *self == RPD::Poll
    }
}
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, RPD>;
impl<'a, REG> RPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Poll the receive descriptor list"]
    #[inline(always)]
    pub fn poll(self) -> &'a mut crate::W<REG> {
        self.variant(RPD::Poll)
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RPD_W<DMARPDRrs> {
        RPD_W::new(self, 0)
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARPDRrs;
impl crate::RegisterSpec for DMARPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpdr::R`](R) reader structure"]
impl crate::Readable for DMARPDRrs {}
#[doc = "`write(|w| ..)` method takes [`dmarpdr::W`](W) writer structure"]
impl crate::Writable for DMARPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARPDR to value 0"]
impl crate::Resettable for DMARPDRrs {
    const RESET_VALUE: u32 = 0;
}
