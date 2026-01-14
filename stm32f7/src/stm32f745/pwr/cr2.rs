///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `CWUPF1` reader - Clear Wakeup Pin flag for PA0
pub type CWUPF1_R = crate::BitReader;
///Field `CWUPF2` reader - Clear Wakeup Pin flag for PA2
pub type CWUPF2_R = crate::BitReader;
///Field `CWUPF3` reader - Clear Wakeup Pin flag for PC1
pub type CWUPF3_R = crate::BitReader;
///Field `CWUPF4` reader - Clear Wakeup Pin flag for PC13
pub type CWUPF4_R = crate::BitReader;
///Field `CWUPF5` reader - Clear Wakeup Pin flag for PI8
pub type CWUPF5_R = crate::BitReader;
///Field `CWUPF6` reader - Clear Wakeup Pin flag for PI11
pub type CWUPF6_R = crate::BitReader;
///Field `WUPP1` reader - Wakeup pin polarity bit for PA0
pub type WUPP1_R = crate::BitReader;
///Field `WUPP1` writer - Wakeup pin polarity bit for PA0
pub type WUPP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP2` reader - Wakeup pin polarity bit for PA2
pub type WUPP2_R = crate::BitReader;
///Field `WUPP2` writer - Wakeup pin polarity bit for PA2
pub type WUPP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP3` reader - Wakeup pin polarity bit for PC1
pub type WUPP3_R = crate::BitReader;
///Field `WUPP3` writer - Wakeup pin polarity bit for PC1
pub type WUPP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP4` reader - Wakeup pin polarity bit for PC13
pub type WUPP4_R = crate::BitReader;
///Field `WUPP4` writer - Wakeup pin polarity bit for PC13
pub type WUPP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP5` reader - Wakeup pin polarity bit for PI8
pub type WUPP5_R = crate::BitReader;
///Field `WUPP5` writer - Wakeup pin polarity bit for PI8
pub type WUPP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPP6` reader - Wakeup pin polarity bit for PI11
pub type WUPP6_R = crate::BitReader;
///Field `WUPP6` writer - Wakeup pin polarity bit for PI11
pub type WUPP6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear Wakeup Pin flag for PA0
    #[inline(always)]
    pub fn cwupf1(&self) -> CWUPF1_R {
        CWUPF1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear Wakeup Pin flag for PA2
    #[inline(always)]
    pub fn cwupf2(&self) -> CWUPF2_R {
        CWUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear Wakeup Pin flag for PC1
    #[inline(always)]
    pub fn cwupf3(&self) -> CWUPF3_R {
        CWUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear Wakeup Pin flag for PC13
    #[inline(always)]
    pub fn cwupf4(&self) -> CWUPF4_R {
        CWUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear Wakeup Pin flag for PI8
    #[inline(always)]
    pub fn cwupf5(&self) -> CWUPF5_R {
        CWUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear Wakeup Pin flag for PI11
    #[inline(always)]
    pub fn cwupf6(&self) -> CWUPF6_R {
        CWUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Wakeup pin polarity bit for PA0
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Wakeup pin polarity bit for PA2
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup pin polarity bit for PC1
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wakeup pin polarity bit for PC13
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup pin polarity bit for PI8
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Wakeup pin polarity bit for PI11
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("cwupf1", &self.cwupf1())
            .field("cwupf2", &self.cwupf2())
            .field("cwupf3", &self.cwupf3())
            .field("cwupf4", &self.cwupf4())
            .field("cwupf5", &self.cwupf5())
            .field("cwupf6", &self.cwupf6())
            .field("wupp1", &self.wupp1())
            .field("wupp2", &self.wupp2())
            .field("wupp3", &self.wupp3())
            .field("wupp4", &self.wupp4())
            .field("wupp5", &self.wupp5())
            .field("wupp6", &self.wupp6())
            .finish()
    }
}
impl W {
    ///Bit 8 - Wakeup pin polarity bit for PA0
    #[inline(always)]
    pub fn wupp1(&mut self) -> WUPP1_W<'_, CR2rs> {
        WUPP1_W::new(self, 8)
    }
    ///Bit 9 - Wakeup pin polarity bit for PA2
    #[inline(always)]
    pub fn wupp2(&mut self) -> WUPP2_W<'_, CR2rs> {
        WUPP2_W::new(self, 9)
    }
    ///Bit 10 - Wakeup pin polarity bit for PC1
    #[inline(always)]
    pub fn wupp3(&mut self) -> WUPP3_W<'_, CR2rs> {
        WUPP3_W::new(self, 10)
    }
    ///Bit 11 - Wakeup pin polarity bit for PC13
    #[inline(always)]
    pub fn wupp4(&mut self) -> WUPP4_W<'_, CR2rs> {
        WUPP4_W::new(self, 11)
    }
    ///Bit 12 - Wakeup pin polarity bit for PI8
    #[inline(always)]
    pub fn wupp5(&mut self) -> WUPP5_W<'_, CR2rs> {
        WUPP5_W::new(self, 12)
    }
    ///Bit 13 - Wakeup pin polarity bit for PI11
    #[inline(always)]
    pub fn wupp6(&mut self) -> WUPP6_W<'_, CR2rs> {
        WUPP6_W::new(self, 13)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#PWR:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
