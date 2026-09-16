///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `ADD` reader - Address of the USART node
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Address of the USART node
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
///LIN break detection interrupt enable
pub use crate::stm32f217::usart1::cr2::LBDIE;
///Field `LBDIE` reader - LIN break detection interrupt enable
pub use crate::stm32f217::usart1::cr2::LBDIE_R;
///Field `LBDIE` writer - LIN break detection interrupt enable
pub use crate::stm32f217::usart1::cr2::LBDIE_W;
///lin break detection length
pub use crate::stm32f217::usart1::cr2::LBDL;
///Field `LBDL` reader - lin break detection length
pub use crate::stm32f217::usart1::cr2::LBDL_R;
///Field `LBDL` writer - lin break detection length
pub use crate::stm32f217::usart1::cr2::LBDL_W;
/**STOP bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    ///0: 1 stop bit
    Stop1 = 0,
    ///2: 2 stop bits
    Stop2 = 2,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
impl crate::IsEnum for STOP {}
///Field `STOP` reader - STOP bits
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOP> {
        match self.bits {
            0 => Some(STOP::Stop1),
            2 => Some(STOP::Stop2),
            _ => None,
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    ///2 stop bits
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
}
///Field `STOP` writer - STOP bits
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    ///2 stop bits
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
}
///LIN mode enable
pub use crate::stm32f217::usart1::cr2::LINEN;
///Field `LINEN` reader - LIN mode enable
pub use crate::stm32f217::usart1::cr2::LINEN_R;
///Field `LINEN` writer - LIN mode enable
pub use crate::stm32f217::usart1::cr2::LINEN_W;
impl R {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("linen", &self.linen())
            .field("stop", &self.stop())
            .field("lbdie", &self.lbdie())
            .field("lbdl", &self.lbdl())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Address of the USART node
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 0)
    }
    ///Bit 5 - lin break detection length
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<'_, CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LIN break detection interrupt enable
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<'_, CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bits 12:13 - STOP bits
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LIN mode enable
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CR2rs> {
        LINEN_W::new(self, 14)
    }
}
/**Control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#UART4:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
