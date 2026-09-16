///Register `DERATEEN` reader
pub type R = crate::R<DERATEENrs>;
///Register `DERATEEN` writer
pub type W = crate::W<DERATEENrs>;
///Field `DERATE_ENABLE` reader - DERATE_ENABLE
pub type DERATE_ENABLE_R = crate::BitReader;
///Field `DERATE_ENABLE` writer - DERATE_ENABLE
pub type DERATE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DERATE_VALUE` reader - DERATE_VALUE
pub type DERATE_VALUE_R = crate::FieldReader;
///Field `DERATE_VALUE` writer - DERATE_VALUE
pub type DERATE_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DERATE_BYTE` reader - DERATE_BYTE
pub type DERATE_BYTE_R = crate::FieldReader;
///Field `DERATE_BYTE` writer - DERATE_BYTE
pub type DERATE_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - DERATE_ENABLE
    #[inline(always)]
    pub fn derate_enable(&self) -> DERATE_ENABLE_R {
        DERATE_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DERATE_VALUE
    #[inline(always)]
    pub fn derate_value(&self) -> DERATE_VALUE_R {
        DERATE_VALUE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:7 - DERATE_BYTE
    #[inline(always)]
    pub fn derate_byte(&self) -> DERATE_BYTE_R {
        DERATE_BYTE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DERATEEN")
            .field("derate_enable", &self.derate_enable())
            .field("derate_value", &self.derate_value())
            .field("derate_byte", &self.derate_byte())
            .finish()
    }
}
impl W {
    ///Bit 0 - DERATE_ENABLE
    #[inline(always)]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W<'_, DERATEENrs> {
        DERATE_ENABLE_W::new(self, 0)
    }
    ///Bits 1:2 - DERATE_VALUE
    #[inline(always)]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W<'_, DERATEENrs> {
        DERATE_VALUE_W::new(self, 1)
    }
    ///Bits 4:7 - DERATE_BYTE
    #[inline(always)]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W<'_, DERATEENrs> {
        DERATE_BYTE_W::new(self, 4)
    }
}
/**DDRCTRL temperature derate enable register

You can [`read`](crate::Reg::read) this register and get [`derateen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`derateen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DERATEEN)*/
pub struct DERATEENrs;
impl crate::RegisterSpec for DERATEENrs {
    type Ux = u32;
}
///`read()` method returns [`derateen::R`](R) reader structure
impl crate::Readable for DERATEENrs {}
///`write(|w| ..)` method takes [`derateen::W`](W) writer structure
impl crate::Writable for DERATEENrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DERATEEN to value 0
impl crate::Resettable for DERATEENrs {}
