///Register `PCONFR` reader
pub type R = crate::R<PCONFRrs>;
///Register `PCONFR` writer
pub type W = crate::W<PCONFRrs>;
/**Number of lanes This field configures the number of active data lanes: Others: Reserved

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NL {
    ///0: One data lane (lane 0)
    B0x0 = 0,
    ///1: Two data lanes (lanes 0 and 1) - Reset value
    B0x1 = 1,
}
impl From<NL> for u8 {
    #[inline(always)]
    fn from(variant: NL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NL {
    type Ux = u8;
}
impl crate::IsEnum for NL {}
///Field `NL` reader - Number of lanes This field configures the number of active data lanes: Others: Reserved
pub type NL_R = crate::FieldReader<NL>;
impl NL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<NL> {
        match self.bits {
            0 => Some(NL::B0x0),
            1 => Some(NL::B0x1),
            _ => None,
        }
    }
    ///One data lane (lane 0)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NL::B0x0
    }
    ///Two data lanes (lanes 0 and 1) - Reset value
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NL::B0x1
    }
}
///Field `NL` writer - Number of lanes This field configures the number of active data lanes: Others: Reserved
pub type NL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, NL>;
impl<'a, REG> NL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///One data lane (lane 0)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NL::B0x0)
    }
    ///Two data lanes (lanes 0 and 1) - Reset value
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NL::B0x1)
    }
}
///Field `SW_TIME` reader - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state.
pub type SW_TIME_R = crate::FieldReader;
///Field `SW_TIME` writer - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state.
pub type SW_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - Number of lanes This field configures the number of active data lanes: Others: Reserved
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:15 - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state.
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCONFR")
            .field("nl", &self.nl())
            .field("sw_time", &self.sw_time())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of lanes This field configures the number of active data lanes: Others: Reserved
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W<PCONFRrs> {
        NL_W::new(self, 0)
    }
    ///Bits 8:15 - Stop wait time This field configures the minimum wait period to request a high-speed transmission after the Stop state.
    #[inline(always)]
    pub fn sw_time(&mut self) -> SW_TIME_W<PCONFRrs> {
        SW_TIME_W::new(self, 8)
    }
}
/**DSI Host PHY configuration register

You can [`read`](crate::Reg::read) this register and get [`pconfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:PCONFR)*/
pub struct PCONFRrs;
impl crate::RegisterSpec for PCONFRrs {
    type Ux = u32;
}
///`read()` method returns [`pconfr::R`](R) reader structure
impl crate::Readable for PCONFRrs {}
///`write(|w| ..)` method takes [`pconfr::W`](W) writer structure
impl crate::Writable for PCONFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCONFR to value 0x01
impl crate::Resettable for PCONFRrs {
    const RESET_VALUE: u32 = 0x01;
}
