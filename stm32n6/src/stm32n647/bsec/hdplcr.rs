///Register `HDPLCR` writer
pub type W = crate::W<HDPLCRrs>;
///Field `INCR_HDPL` writer - Increment HDPL
pub type INCR_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<HDPLCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Increment HDPL
    #[inline(always)]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<'_, HDPLCRrs> {
        INCR_HDPL_W::new(self, 0)
    }
}
/**BSEC HDPL control

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#BSEC:HDPLCR)*/
pub struct HDPLCRrs;
impl crate::RegisterSpec for HDPLCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hdplcr::W`](W) writer structure
impl crate::Writable for HDPLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HDPLCR to value 0
impl crate::Resettable for HDPLCRrs {}
