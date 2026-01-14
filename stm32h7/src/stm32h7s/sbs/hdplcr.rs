///Register `HDPLCR` reader
pub type R = crate::R<HDPLCRrs>;
///Register `HDPLCR` writer
pub type W = crate::W<HDPLCRrs>;
///Field `INCR_HDPL` reader - increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4).
pub type INCR_HDPL_R = crate::FieldReader;
///Field `INCR_HDPL` writer - increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4).
pub type INCR_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4).
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
    ///Bits 0:7 - increment HDPL Write 0x6A to increment device HDPL by one. After a write, the register value reverts to its default value (0xB4).
    #[inline(always)]
    pub fn incr_hdpl(&mut self) -> INCR_HDPL_W<'_, HDPLCRrs> {
        INCR_HDPL_W::new(self, 0)
    }
}
/**SBS hide protection control register

You can [`read`](crate::Reg::read) this register and get [`hdplcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:HDPLCR)*/
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
