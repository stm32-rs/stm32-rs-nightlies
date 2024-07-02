///Register `DDRCTRL_DERATEEN` reader
pub type R = crate::R<DDRCTRL_DERATEENrs>;
///Register `DDRCTRL_DERATEEN` writer
pub type W = crate::W<DDRCTRL_DERATEENrs>;
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
        f.debug_struct("DDRCTRL_DERATEEN")
            .field("derate_enable", &self.derate_enable())
            .field("derate_value", &self.derate_value())
            .field("derate_byte", &self.derate_byte())
            .finish()
    }
}
impl W {
    ///Bit 0 - DERATE_ENABLE
    #[inline(always)]
    #[must_use]
    pub fn derate_enable(&mut self) -> DERATE_ENABLE_W<DDRCTRL_DERATEENrs> {
        DERATE_ENABLE_W::new(self, 0)
    }
    ///Bits 1:2 - DERATE_VALUE
    #[inline(always)]
    #[must_use]
    pub fn derate_value(&mut self) -> DERATE_VALUE_W<DDRCTRL_DERATEENrs> {
        DERATE_VALUE_W::new(self, 1)
    }
    ///Bits 4:7 - DERATE_BYTE
    #[inline(always)]
    #[must_use]
    pub fn derate_byte(&mut self) -> DERATE_BYTE_W<DDRCTRL_DERATEENrs> {
        DERATE_BYTE_W::new(self, 4)
    }
}
/**DDRCTRL temperature derate enable register

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_derateen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_derateen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DDRCTRL_DERATEEN)*/
pub struct DDRCTRL_DERATEENrs;
impl crate::RegisterSpec for DDRCTRL_DERATEENrs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_derateen::R`](R) reader structure
impl crate::Readable for DDRCTRL_DERATEENrs {}
///`write(|w| ..)` method takes [`ddrctrl_derateen::W`](W) writer structure
impl crate::Writable for DDRCTRL_DERATEENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_DERATEEN to value 0
impl crate::Resettable for DDRCTRL_DERATEENrs {
    const RESET_VALUE: u32 = 0;
}
