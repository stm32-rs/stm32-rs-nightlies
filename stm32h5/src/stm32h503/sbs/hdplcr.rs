///Register `HDPLCR` reader
pub type R = crate::R<HDPLCRrs>;
///Register `HDPLCR` writer
pub type W = crate::W<HDPLCRrs>;
/**increment HDPL value Other: all other values allow a HDPL level increment.

Value on reset: 180*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCR_HDPL {
    ///106: Increment HDPL value
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
impl crate::IsEnum for INCR_HDPL {}
///Field `INCR_HDPL` reader - increment HDPL value Other: all other values allow a HDPL level increment.
pub type INCR_HDPL_R = crate::FieldReader<INCR_HDPL>;
impl INCR_HDPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<INCR_HDPL> {
        match self.bits {
            106 => Some(INCR_HDPL::Increment),
            _ => None,
        }
    }
    ///Increment HDPL value
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == INCR_HDPL::Increment
    }
}
///Field `INCR_HDPL` writer - increment HDPL value Other: all other values allow a HDPL level increment.
pub type INCR_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, INCR_HDPL>;
impl<'a, REG> INCR_HDPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Increment HDPL value
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(INCR_HDPL::Increment)
    }
}
impl R {
    ///Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment.
    #[inline(always)]
    pub fn incr_hdpl(&self) -> INCR_HDPL_R {
        INCR_HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPLCR")
            .field("incr_hdpl", &self.incr_hdpl())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - increment HDPL value Other: all other values allow a HDPL level increment.
    #[inline(always)]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<'_, HDPLCRrs> {
        INCR_HDPL_W::new(self, 0)
    }
}
/**SBS temporal isolation control register

You can [`read`](crate::Reg::read) this register and get [`hdplcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:HDPLCR)*/
pub struct HDPLCRrs;
impl crate::RegisterSpec for HDPLCRrs {
    type Ux = u32;
}
///`read()` method returns [`hdplcr::R`](R) reader structure
impl crate::Readable for HDPLCRrs {}
///`write(|w| ..)` method takes [`hdplcr::W`](W) writer structure
impl crate::Writable for HDPLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDPLCR to value 0xb4
impl crate::Resettable for HDPLCRrs {
    const RESET_VALUE: u32 = 0xb4;
}
