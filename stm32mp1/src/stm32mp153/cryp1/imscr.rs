///Register `IMSCR` reader
pub type R = crate::R<IMSCRrs>;
///Register `IMSCR` writer
pub type W = crate::W<IMSCRrs>;
///Field `INIM` reader - INIM
pub type INIM_R = crate::BitReader;
///Field `INIM` writer - INIM
pub type INIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTIM` reader - OUTIM
pub type OUTIM_R = crate::BitReader;
///Field `OUTIM` writer - OUTIM
pub type OUTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - INIM
    #[inline(always)]
    pub fn inim(&self) -> INIM_R {
        INIM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OUTIM
    #[inline(always)]
    pub fn outim(&self) -> OUTIM_R {
        OUTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSCR")
            .field("inim", &self.inim())
            .field("outim", &self.outim())
            .finish()
    }
}
impl W {
    ///Bit 0 - INIM
    #[inline(always)]
    pub fn inim(&mut self) -> INIM_W<'_, IMSCRrs> {
        INIM_W::new(self, 0)
    }
    ///Bit 1 - OUTIM
    #[inline(always)]
    pub fn outim(&mut self) -> OUTIM_W<'_, IMSCRrs> {
        OUTIM_W::new(self, 1)
    }
}
/**The CRYP_IMSCR register is the interrupt mask set or clear register. It is a read/write register. When a read operation is performed, this register gives the current value of the mask applied to the relevant interrupt. Writing 1 to the particular bit sets the mask, thus enabling the interrupt to be read. Writing 0 to this bit clears the corresponding mask. All the bits are cleared to 0 when the peripheral is reset.

You can [`read`](crate::Reg::read) this register and get [`imscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:IMSCR)*/
pub struct IMSCRrs;
impl crate::RegisterSpec for IMSCRrs {
    type Ux = u32;
}
///`read()` method returns [`imscr::R`](R) reader structure
impl crate::Readable for IMSCRrs {}
///`write(|w| ..)` method takes [`imscr::W`](W) writer structure
impl crate::Writable for IMSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IMSCR to value 0
impl crate::Resettable for IMSCRrs {}
