///Register `ZQ0SR0` reader
pub type R = crate::R<ZQ0SR0rs>;
///Field `ZCTRL` reader - ZCTRL
pub type ZCTRL_R = crate::FieldReader<u32>;
///Field `ZERR` reader - ZERR
pub type ZERR_R = crate::BitReader;
///Field `ZDONE` reader - ZDONE
pub type ZDONE_R = crate::BitReader;
impl R {
    ///Bits 0:19 - ZCTRL
    #[inline(always)]
    pub fn zctrl(&self) -> ZCTRL_R {
        ZCTRL_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 30 - ZERR
    #[inline(always)]
    pub fn zerr(&self) -> ZERR_R {
        ZERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ZDONE
    #[inline(always)]
    pub fn zdone(&self) -> ZDONE_R {
        ZDONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZQ0SR0")
            .field("zctrl", &self.zctrl())
            .field("zerr", &self.zerr())
            .field("zdone", &self.zdone())
            .finish()
    }
}
/**DDRPHYC ZQ0S register 0

You can [`read`](crate::Reg::read) this register and get [`zq0sr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:ZQ0SR0)*/
pub struct ZQ0SR0rs;
impl crate::RegisterSpec for ZQ0SR0rs {
    type Ux = u32;
}
///`read()` method returns [`zq0sr0::R`](R) reader structure
impl crate::Readable for ZQ0SR0rs {}
///`reset()` method sets ZQ0SR0 to value 0x014a
impl crate::Resettable for ZQ0SR0rs {
    const RESET_VALUE: u32 = 0x014a;
}
