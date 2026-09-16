///Register `MRCTRL0` reader
pub type R = crate::R<MRCTRL0rs>;
///Register `MRCTRL0` writer
pub type W = crate::W<MRCTRL0rs>;
///Field `MR_TYPE` reader - MR_TYPE
pub type MR_TYPE_R = crate::BitReader;
///Field `MR_TYPE` writer - MR_TYPE
pub type MR_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR_RANK` reader - MR_RANK
pub type MR_RANK_R = crate::BitReader;
///Field `MR_RANK` writer - MR_RANK
pub type MR_RANK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MR_ADDR` reader - MR_ADDR
pub type MR_ADDR_R = crate::FieldReader;
///Field `MR_ADDR` writer - MR_ADDR
pub type MR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MR_WR` reader - MR_WR
pub type MR_WR_R = crate::BitReader;
///Field `MR_WR` writer - MR_WR
pub type MR_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - MR_TYPE
    #[inline(always)]
    pub fn mr_type(&self) -> MR_TYPE_R {
        MR_TYPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - MR_RANK
    #[inline(always)]
    pub fn mr_rank(&self) -> MR_RANK_R {
        MR_RANK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 12:15 - MR_ADDR
    #[inline(always)]
    pub fn mr_addr(&self) -> MR_ADDR_R {
        MR_ADDR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 31 - MR_WR
    #[inline(always)]
    pub fn mr_wr(&self) -> MR_WR_R {
        MR_WR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRCTRL0")
            .field("mr_type", &self.mr_type())
            .field("mr_rank", &self.mr_rank())
            .field("mr_addr", &self.mr_addr())
            .field("mr_wr", &self.mr_wr())
            .finish()
    }
}
impl W {
    ///Bit 0 - MR_TYPE
    #[inline(always)]
    pub fn mr_type(&mut self) -> MR_TYPE_W<'_, MRCTRL0rs> {
        MR_TYPE_W::new(self, 0)
    }
    ///Bit 4 - MR_RANK
    #[inline(always)]
    pub fn mr_rank(&mut self) -> MR_RANK_W<'_, MRCTRL0rs> {
        MR_RANK_W::new(self, 4)
    }
    ///Bits 12:15 - MR_ADDR
    #[inline(always)]
    pub fn mr_addr(&mut self) -> MR_ADDR_W<'_, MRCTRL0rs> {
        MR_ADDR_W::new(self, 12)
    }
    ///Bit 31 - MR_WR
    #[inline(always)]
    pub fn mr_wr(&mut self) -> MR_WR_W<'_, MRCTRL0rs> {
        MR_WR_W::new(self, 31)
    }
}
/**Mode Register Read/Write Control Register 0. Do not enable more than one of the following fields simultaneously: sw_init_int pda_en mpr_en

You can [`read`](crate::Reg::read) this register and get [`mrctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:MRCTRL0)*/
pub struct MRCTRL0rs;
impl crate::RegisterSpec for MRCTRL0rs {
    type Ux = u32;
}
///`read()` method returns [`mrctrl0::R`](R) reader structure
impl crate::Readable for MRCTRL0rs {}
///`write(|w| ..)` method takes [`mrctrl0::W`](W) writer structure
impl crate::Writable for MRCTRL0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MRCTRL0 to value 0x10
impl crate::Resettable for MRCTRL0rs {
    const RESET_VALUE: u32 = 0x10;
}
