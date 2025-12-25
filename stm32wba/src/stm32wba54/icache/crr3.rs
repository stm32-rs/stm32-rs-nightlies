///Register `CRR3` reader
pub type R = crate::R<CRR3rs>;
///Register `CRR3` writer
pub type W = crate::W<CRR3rs>;
///Field `BASEADDR` reader - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
pub type BASEADDR_R = crate::FieldReader;
///Field `BASEADDR` writer - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
pub type BASEADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSIZE` reader - size for region x
pub type RSIZE_R = crate::FieldReader;
///Field `RSIZE` writer - size for region x
pub type RSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REN` reader - enable for region x
pub type REN_R = crate::BitReader;
///Field `REN` writer - enable for region x
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REMAPADDR` reader - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
pub type REMAPADDR_R = crate::FieldReader<u16>;
///Field `REMAPADDR` writer - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
pub type REMAPADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `MSTSEL` reader - AHB cache master selection for region x
pub type MSTSEL_R = crate::BitReader;
///Field `MSTSEL` writer - AHB cache master selection for region x
pub type MSTSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBURST` reader - output burst type for region x
pub type HBURST_R = crate::BitReader;
///Field `HBURST` writer - output burst type for region x
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 9:11 - size for region x
    #[inline(always)]
    pub fn rsize(&self) -> RSIZE_R {
        RSIZE_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 15 - enable for region x
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn remapaddr(&self) -> REMAPADDR_R {
        REMAPADDR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    ///Bit 28 - AHB cache master selection for region x
    #[inline(always)]
    pub fn mstsel(&self) -> MSTSEL_R {
        MSTSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - output burst type for region x
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRR3")
            .field("baseaddr", &self.baseaddr())
            .field("rsize", &self.rsize())
            .field("ren", &self.ren())
            .field("remapaddr", &self.remapaddr())
            .field("mstsel", &self.mstsel())
            .field("hburst", &self.hburst())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - base address for region x This alias address is replaced by REMAPADDR field. The only useful bits are \[28:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W<'_, CRR3rs> {
        BASEADDR_W::new(self, 0)
    }
    ///Bits 9:11 - size for region x
    #[inline(always)]
    pub fn rsize(&mut self) -> RSIZE_W<'_, CRR3rs> {
        RSIZE_W::new(self, 9)
    }
    ///Bit 15 - enable for region x
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W<'_, CRR3rs> {
        REN_W::new(self, 15)
    }
    ///Bits 16:26 - remapped address for region x This field replaces the alias address defined by BASEADDR field. The only useful bits are \[31:RI\], where 21 less than or equal RI less than or equal 27 is the number of bits of RSIZE (see Section 18.4.7). If the programmed value has more LSBs, the useless bits are ignored.
    #[inline(always)]
    pub fn remapaddr(&mut self) -> REMAPADDR_W<'_, CRR3rs> {
        REMAPADDR_W::new(self, 16)
    }
    ///Bit 28 - AHB cache master selection for region x
    #[inline(always)]
    pub fn mstsel(&mut self) -> MSTSEL_W<'_, CRR3rs> {
        MSTSEL_W::new(self, 28)
    }
    ///Bit 31 - output burst type for region x
    #[inline(always)]
    pub fn hburst(&mut self) -> HBURST_W<'_, CRR3rs> {
        HBURST_W::new(self, 31)
    }
}
/**ICACHE region 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`crr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#ICACHE:CRR3)*/
pub struct CRR3rs;
impl crate::RegisterSpec for CRR3rs {
    type Ux = u32;
}
///`read()` method returns [`crr3::R`](R) reader structure
impl crate::Readable for CRR3rs {}
///`write(|w| ..)` method takes [`crr3::W`](W) writer structure
impl crate::Writable for CRR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CRR3 to value 0x0200
impl crate::Resettable for CRR3rs {
    const RESET_VALUE: u32 = 0x0200;
}
