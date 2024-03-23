#[doc = "Register `HDPLCR` reader"]
pub type R = crate::R<HDPLCRrs>;
#[doc = "Register `HDPLCR` writer"]
pub type W = crate::W<HDPLCRrs>;
#[doc = "increment HDPL value Other: all other values allow a HDPL level increment.\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCR_HDPL {
    #[doc = "106: Increment HDPL value"]
    Increment = 106,
}
impl From<INCR_HDPL> for u8 {
    #[inline(always)]
    fn from(variant: INCR_HDPL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INCR_HDPL {
    type Ux = u8;
}
#[doc = "Field `INCR_HDPL` reader - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_R = crate::FieldReader<INCR_HDPL>;
impl INCR_HDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INCR_HDPL> {
        match self.bits {
            106 => Some(INCR_HDPL::Increment),
            _ => None,
        }
    }
    #[doc = "Increment HDPL value"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == INCR_HDPL::Increment
    }
}
#[doc = "Field `INCR_HDPL` writer - increment HDPL value Other: all other values allow a HDPL level increment."]
pub type INCR_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, INCR_HDPL>;
impl<'a, REG> INCR_HDPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment HDPL value"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(INCR_HDPL::Increment)
    }
}
impl R {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    pub fn incr_hdpl(&self) -> INCR_HDPL_R {
        INCR_HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment."]
    #[inline(always)]
    #[must_use]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<HDPLCRrs> {
        INCR_HDPL_W::new(self, 0)
    }
}
#[doc = "SBS temporal isolation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdplcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdplcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPLCRrs;
impl crate::RegisterSpec for HDPLCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdplcr::R`](R) reader structure"]
impl crate::Readable for HDPLCRrs {}
#[doc = "`write(|w| ..)` method takes [`hdplcr::W`](W) writer structure"]
impl crate::Writable for HDPLCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDPLCR to value 0xb4"]
impl crate::Resettable for HDPLCRrs {
    const RESET_VALUE: u32 = 0xb4;
}
